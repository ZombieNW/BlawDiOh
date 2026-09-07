use crate::types::{MouthFrame, MouthState};

pub struct Mouth {}

impl Mouth {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_mouth(&mut self, rms: &f32) -> MouthFrame {
        let state = match rms {
            x if *x < 0.02 => MouthState::Closed,
            x if *x < 0.05 => MouthState::Small,
            x if *x < 0.1 => MouthState::Half,
            x if *x < 0.2 => MouthState::Wide,
            _ => MouthState::Open,
        };

        return MouthFrame {
            offset_x: 0,
            offset_y: 0,
            state,
        };
    }
}
