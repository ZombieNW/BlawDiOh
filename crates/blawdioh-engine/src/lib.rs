use crate::blinker::Blinker;

mod blinker;
mod sampler;

#[derive(Clone, Copy, PartialEq, Eq)]
enum MouthState {
    Closed,
    Small,
    Half,
    Wide,
    Open,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum EyeState {
    Open,
    Blink,
    HalfBlink,
}

pub struct KeyFrame {
    frame: u32,
    mouth: MouthState,
    eye: EyeState,
}

pub fn generate_keyframes(rms_frames: &[f32]) -> Vec<KeyFrame> {
    let mut keyframes: Vec<KeyFrame> = Vec::with_capacity(rms_frames.len());
    let mut blinker = Blinker::new();

    for (frame, rms) in rms_frames.iter().enumerate() {
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

fn get_mouth(rms: &f32) -> MouthState {
    match rms {
        x if *x < 0.05 => MouthState::Closed,
        x if *x < 0.2 => MouthState::Small,
        x if *x < 0.4 => MouthState::Half,
        x if *x < 0.6 => MouthState::Wide,
        _ => MouthState::Open,
    }
}

/// Smooths a vector of RMS values using the Exponential Moving Average algorithm
/// Alpha (0 -> 1) is responseiveness
fn smooth(values: &[f32], alpha: f32) -> Vec<f32> {
    if values.is_empty() {
        return Vec::new();
    }

    let mut smoothed_values = Vec::with_capacity(values.len());
    let mut current_smoothed = values[0];
    smoothed_values.push(current_smoothed);

    for &raw_value in values.iter().skip(1) {
        current_smoothed = (alpha * raw_value) + ((1.0 - alpha) * current_smoothed);
        smoothed_values.push(current_smoothed);
    }

    return smoothed_values;
}
