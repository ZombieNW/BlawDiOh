use crate::assets::AssetBundle;
use blawdioh_engine::types::KeyFrame;
use image::RgbaImage;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use std::{
    error::Error,
    io::Write,
    path::Path,
    process::{Child, Command, Stdio},
};

pub mod assets;

pub fn render_video<F>(
    assets: &AssetBundle,
    keyframes: &[KeyFrame],
    audio_path: &Path,
    output_path: &Path,
    fps: f64,
    mut on_frame_rendered: F,
) -> Result<(), Box<dyn Error>>
where
    F: FnMut(),
{
    if keyframes.is_empty() {
        return Ok(());
    }

    let mut ffmpeg = spawn_ffmpeg(assets, audio_path, output_path, fps)?;
    let mut stdin = ffmpeg.stdin.take().ok_or("Failed to open FFmpeg stdin")?;

    // Chunk frames to render in parallel
    let chunk_size = rayon::current_num_threads() * 4;

    for chunk in keyframes.chunks(chunk_size) {
        // Generate frames in parallel
        let rendered_chunk: Vec<RgbaImage> = chunk
            .par_iter()
            .map(|keyframe| render_frame(assets, keyframe))
            .collect();

        // Pipe frames to ffmpeg
        for frame in rendered_chunk {
            stdin.write_all(frame.as_raw())?;
            on_frame_rendered();
        }
    }

    drop(stdin);

    // Check output for errors
    let output = ffmpeg.wait_with_output()?;
    if !output.status.success() {
        return Err(format!(
            "FFmpeg failed with exit code {:?}:\n{}",
            output.status.code(),
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(())
}

/// Spawn ffmpeg child process
fn spawn_ffmpeg(
    assets: &AssetBundle,
    audio_path: &Path,
    output_path: &Path,
    fps: f64,
) -> Result<Child, std::io::Error> {
    let (w, h) = (assets.base_texture.width(), assets.base_texture.height());

    Command::new("ffmpeg")
        .args([
            "-y",
            "-loglevel",
            "error",
            "-f",
            "rawvideo",
            "-pixel_format",
            "rgba",
            "-video_size",
            &format!("{w}x{h}"),
            "-framerate",
            &fps.to_string(),
            "-i",
            "pipe:0",
            "-i",
        ])
        .arg(audio_path)
        .args([
            "-c:v",
            "libx264",
            "-pix_fmt",
            "yuv420p",
            "-c:a",
            "aac",
            "-shortest",
        ])
        .arg(output_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
}

/// Generate face texture from keyframe
pub fn render_frame(assets: &AssetBundle, keyframe: &KeyFrame) -> RgbaImage {
    let mut frame = assets.base_texture.clone();

    // Overlay mouth texture onto base texture
    if let Some(mouth_sprite) = assets.mouths.get(&keyframe.mouth.state) {
        image::imageops::overlay(
            &mut frame,
            mouth_sprite,
            keyframe.mouth.offset_x as i64,
            keyframe.mouth.offset_y as i64,
        );
    }

    // Overlay eyes texture onto base texture
    if let Some(eye_sprite) = assets.eyes.get(&keyframe.eye.state) {
        image::imageops::overlay(
            &mut frame,
            eye_sprite,
            keyframe.eye.offset_x as i64,
            keyframe.eye.offset_y as i64,
        );
    }

    return frame;
}
