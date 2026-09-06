use std::path::Path;

use crate::blinker::Blinker;

mod blinker;
mod math;
mod sampler;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum MouthState {
    Closed,
    Small,
    Half,
    Wide,
    Open,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EyeState {
    Open,
    Blink,
    HalfBlink,
}

#[derive(Debug)]
pub struct MouthFrame {
    offset_x: u32,
    offset_y: u32,
    state: MouthState,
}

#[derive(Debug)]
pub struct EyeFrame {
    offset_x: u32,
    offset_y: u32,
    state: EyeState,
}

#[derive(Debug)]
pub struct KeyFrame {
    frame: u32,
    mouth: MouthFrame,
    eye: EyeFrame,
}

pub fn generate_keyframes(path: &Path, fps: f64) -> Vec<KeyFrame> {
    let samples = sampler::rms_at_fps(path, fps).unwrap();
    let smoothed_samples = math::smooth(&samples, 0.5);

    let mut keyframes: Vec<KeyFrame> = Vec::with_capacity(smoothed_samples.len());
    let mut blinker = Blinker::new();

    for (frame, rms) in smoothed_samples.iter().enumerate() {
        let eye = blinker.get_eye();
        let mouth = get_mouth(rms);

        let keyframe = KeyFrame {
            frame: frame.try_into().unwrap(),
            mouth: mouth,
            eye: eye,
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
