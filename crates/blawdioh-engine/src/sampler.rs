use std::{error::Error, fs::File, path::Path};

use symphonia::core::{
    codecs::audio::AudioDecoderOptions,
    formats::{FormatOptions, probe::Hint},
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

    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    };

    let mut format = symphonia::default::get_probe().probe(
        &hint,
        mss,
        FormatOptions::default(),
        MetadataOptions::default(),
    )?;

    let track = format
        .default_track(symphonia::core::formats::TrackType::Audio)
        .ok_or("no audio track found")?;
    let track_id = track.id;
    let track_codec_params = track.codec_params.clone().ok_or("missing track codec")?;
    let track_params = track_codec_params
        .audio()
        .ok_or("missing track audio params")?;
    let sample_rate = track_params.sample_rate.ok_or("missing sample rate")?;

    let mut decoder = symphonia::default::get_codecs()
        .make_audio_decoder(track_params, &AudioDecoderOptions::default())?;

    let mut mono_samples: Vec<f32> = Vec::new();
    let mut sample_buf: Vec<f32> = Vec::new();

    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(symphonia::core::errors::Error::IoError(e))
                if e.kind() == std::io::ErrorKind::UnexpectedEof =>
            {
                break;
            }
            Err(e) => return Err(e.into()),
        }
        .unwrap();

        if packet.track_id != track_id {
            continue;
        }

        let decoded = match decoder.decode(&packet) {
            Ok(decoded) => decoded,
            Err(symphonia::core::errors::Error::IoError(e)) => {
                continue;
            }
            Err(e) => return Err(e.into()),
        };

        decoded.copy_to_vec_interleaved(&mut sample_buf);

        let channels = decoded.spec().channels().count();

        for frame in sample_buf.chunks_exact(channels) {
            mono_samples.push(frame.iter().sum::<f32>() / channels as f32);
        }
    }

    Ok((mono_samples, sample_rate))
}

fn chunk_into_rms(samples: &[f32], sample_rate: u32, fps: f64) -> Vec<f32> {
    if samples.is_empty() || sample_rate == 0 {
        return Vec::new();
    }

    let samples_per_frame = ((sample_rate as f64 / fps).round() as usize).max(1);

    return samples
        .chunks(samples_per_frame)
        .map(|chunk| {
            let sum_sq: f32 = chunk.iter().map(|s| s * s).sum();
            return (sum_sq / chunk.len() as f32).sqrt();
        })
        .collect();
}
