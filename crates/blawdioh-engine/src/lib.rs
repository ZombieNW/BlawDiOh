use std::path::Path;

use crate::{
    eye::Eye,
    types::{KeyFrame, MouthFrame, MouthState},
};

mod eye;
mod math;
mod sampler;
pub mod types;

pub fn generate_keyframes(path: &Path, fps: f64) -> Vec<KeyFrame> {
    let samples = sampler::rms_at_fps(path, fps).unwrap();
    let smoothed_samples = math::smooth(&samples, 0.5);

    let mut keyframes: Vec<KeyFrame> = Vec::with_capacity(smoothed_samples.len());
    let mut eye = Eye::new();

    for (frame, rms) in smoothed_samples.iter().enumerate() {
        let keyframe = KeyFrame {
            frame: frame.try_into().unwrap(),
            mouth: get_mouth(rms),
            eye: eye.get_eye(),
        };

        keyframes.push(keyframe);
    }
    return keyframes;
}

fn get_mouth(rms: &f32) -> MouthFrame {
    let state = match rms {
        x if *x < 0.05 => MouthState::Closed,
        x if *x < 0.2 => MouthState::Small,
        x if *x < 0.4 => MouthState::Half,
        x if *x < 0.6 => MouthState::Wide,
        _ => MouthState::Open,
    };

    return MouthFrame {
        offset_x: 0,
        offset_y: 0,
        state,
    };
}
