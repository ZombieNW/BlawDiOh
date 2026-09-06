use std::{error::Error, fs::File, path::Path};

use symphonia::core::{
    codecs::audio::AudioDecoderOptions,
    formats::{FormatOptions, TrackType, probe::Hint},
    io::MediaSourceStream,
    meta::MetadataOptions,
};

pub fn rms_at_fps(path: &Path, fps: f64) -> Result<Vec<f32>, Box<dyn Error>> {
    let (samples, sample_rate) = decode_audio(path)?;
    Ok(chunk_into_rms(&samples, sample_rate, fps))
}

fn decode_audio(path: &Path) -> Result<(Vec<f32>, u32), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    // figure out what file type it is based on extension
    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    };

    let probe = symphonia::default::get_probe();
    let mut format = probe.probe(
        &hint,
        mss,
        FormatOptions::default(),
        MetadataOptions::default(),
    )?;

    let track = format
        .default_track(TrackType::Audio)
        .ok_or("no audio track found")?;

    let track_id = track.id;
    let codec_params = track.codec_params.clone().ok_or("missing track codec")?;
    let track_params = codec_params.audio().ok_or("missing track audio params")?;
    let sample_rate = track_params.sample_rate.ok_or("missing sample rate")?;

    let mut decoder = symphonia::default::get_codecs()
        .make_audio_decoder(track_params, &AudioDecoderOptions::default())?;

    let mut mono_samples: Vec<f32> = Vec::new();
    let mut sample_buf: Vec<f32> = Vec::new();

    // cycle through each packet
    loop {
        match format.next_packet() {
            Ok(packet) => {
                let Some(packet) = packet else {
                    break; // eof
                };

                if packet.track_id != track_id {
                    continue;
                }

                let decoded = decoder.decode(&packet)?;
                decoded.copy_to_vec_interleaved(&mut sample_buf);

                let channels = decoded.spec().channels().count();
                for frame in sample_buf.chunks_exact(channels) {
                    mono_samples.push(frame.iter().sum::<f32>() / channels as f32);
                }
                sample_buf.clear();
            }
            Err(e) => {
                if let symphonia::core::errors::Error::IoError(ref io_err) = e {
                    if io_err.kind() == std::io::ErrorKind::UnexpectedEof {
                        break;
                    }
                }
                return Err(e.into());
            }
        }
    }

    Ok((mono_samples, sample_rate))
}

fn chunk_into_rms(samples: &[f32], sample_rate: u32, fps: f64) -> Vec<f32> {
    if samples.is_empty() {
        return Vec::new();
    }

    let samples_per_frame = ((sample_rate as f64 / fps).round() as usize).max(1);
    return samples
        .chunks(samples_per_frame)
        .map(|chunk| {
            let sum_sq = chunk.iter().map(|s| s * s).sum::<f32>();
            (sum_sq / chunk.len() as f32).sqrt()
        })
        .collect();
}
