use crate::types::{EyeFrame, EyeState};

pub struct Eye {
    animation_frame: usize,
    cooldown: u32,
}

impl Eye {
    pub fn new() -> Self {
        Self {
            animation_frame: EyeState::BLINK.len(),
            cooldown: Self::random_interval(),
        }
    }

    pub fn get_eye(&mut self) -> EyeFrame {
        let state = self.get_state();

        return EyeFrame {
            offset_x: 0,
            offset_y: 0,
            state: state,
        };
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
