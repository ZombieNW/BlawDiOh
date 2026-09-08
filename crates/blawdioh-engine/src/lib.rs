use std::path::Path;

use crate::{eye::Eye, mouth::Mouth, types::KeyFrame};

mod eye;
mod math;
mod mouth;
mod sampler;
pub mod types;

pub fn generate_keyframes(path: &Path, fps: f64) -> Vec<KeyFrame> {
    let samples = sampler::rms_at_fps(path, fps).unwrap();
    let smoothed_samples = math::smooth(&samples, 0.5);

    let mut keyframes: Vec<KeyFrame> = Vec::with_capacity(smoothed_samples.len());
    let mut eye = Eye::new();
    let mut mouth = Mouth::new();

    for (frame, rms) in smoothed_samples.iter().enumerate() {
        let keyframe = KeyFrame {
            frame: frame.try_into().unwrap(),
            mouth: mouth.get_mouth(rms, frame),
            eye: eye.get_eye(frame),
        };

        keyframes.push(keyframe);
    }
    return keyframes;
}
