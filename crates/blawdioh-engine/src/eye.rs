use crate::types::{EyeFrame, EyeState};
use noise::{NoiseFn, Perlin};

pub struct Eye {
    animation_frame: usize,
    cooldown: u32,
    perlin: Perlin,
}

impl Eye {
    pub fn new() -> Self {
        Self {
            animation_frame: EyeState::BLINK.len(),
            cooldown: Self::random_interval(),
            perlin: Perlin::new(rand::random()),
        }
    }

    pub fn get_eye(&mut self, frame: usize) -> EyeFrame {
        let t = frame as f64 / (24.0 * 2.5);

        let offset_x = self.noise(t, 0.0, 0, 10);
        let offset_y = self.noise(t, 10.0, 0, 10);

        return EyeFrame {
            offset_x,
            offset_y,
            state: self.get_state(),
        };
    }

    fn noise(&self, t: f64, track: f64, min: u32, max: u32) -> u32 {
        let raw = self.perlin.get([t, track]);
        let norm = ((raw + 1.0) * 0.5).clamp(0.0, 1.0);
        min + (norm * (max - min) as f64).round() as u32
    }

    fn get_state(&mut self) -> EyeState {
        // Play current animation
        if self.animation_frame < EyeState::BLINK.len() {
            let state = EyeState::BLINK[self.animation_frame];
            self.animation_frame += 1;
            return state;
        }

        // Decrement cooldown
        if self.cooldown > 0 {
            self.cooldown -= 1;
            return EyeState::Open;
        }

        // Start new animation after cooldown
        self.cooldown = Self::random_interval();
        self.animation_frame = 1;
        return EyeState::BLINK[0];
    }

    fn random_interval() -> u32 {
        rand::random_range(36..96)
    }
}
