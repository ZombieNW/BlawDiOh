use crate::types::{MouthFrame, MouthState};
use noise::{NoiseFn, Perlin};

pub struct Mouth {
    perlin: Perlin,
}

impl Mouth {
    pub fn new() -> Self {
        Self {
            perlin: Perlin::new(rand::random()),
        }
    }

    pub fn get_mouth(&mut self, rms: &f32, frame: usize) -> MouthFrame {
        let t = frame as f64 / (24.0 * 2.5);

        let offset_x = self.noise(t, 0.0, 0, 10);
        let offset_y = self.noise(t, 10.0, 0, 10);

        return MouthFrame {
            offset_x,
            offset_y,
            state: self.get_state(rms),
        };
    }

    fn get_state(&self, rms: &f32) -> MouthState {
        match rms {
            x if *x < 0.02 => MouthState::Closed,
            x if *x < 0.05 => MouthState::Small,
            x if *x < 0.1 => MouthState::Half,
            x if *x < 0.2 => MouthState::Wide,
            _ => MouthState::Open,
        }
    }

    fn noise(&self, t: f64, track: f64, min: u32, max: u32) -> u32 {
        let raw = self.perlin.get([t, track]);
        let norm = ((raw + 1.0) * 0.5).clamp(0.0, 1.0);
        min + (norm * (max - min) as f64).round() as u32
    }
}
