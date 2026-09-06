use crate::EyeState;

pub struct Blinker {
    animation_frame: usize,
    cooldown: u32,
}

const BLINK_ANIMATION: [EyeState; 3] = [EyeState::HalfBlink, EyeState::Blink, EyeState::HalfBlink];

impl Blinker {
    pub fn new() -> Self {
        Self {
            animation_frame: 0,
            cooldown: Self::random_interval(),
        }
    }

    fn random_interval() -> u32 {
        rand::random_range(24..72)
    }

    pub fn get_eye(&mut self) -> EyeState {
        // run through already running animation
        if self.animation_frame < BLINK_ANIMATION.len() {
            let state = BLINK_ANIMATION[self.animation_frame];
            self.animation_frame += 1;
            return state;
        }

        // lower cooldown
        if self.cooldown > 0 {
            self.cooldown -= 1;
            return EyeState::Open;
        }

        // start animation when cooldown over
        self.cooldown = Self::random_interval();
        self.animation_frame = 1;
        return BLINK_ANIMATION[0];
    }
}
