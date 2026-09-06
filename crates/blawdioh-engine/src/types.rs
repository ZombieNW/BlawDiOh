#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MouthState {
    Closed,
    Small,
    Half,
    Wide,
    Open,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum EyeState {
    Open,
    Blink,
    HalfBlink,
}

#[derive(Debug)]
pub struct MouthFrame {
    pub offset_x: u32,
    pub offset_y: u32,
    pub state: MouthState,
}

#[derive(Debug)]
pub struct EyeFrame {
    pub offset_x: u32,
    pub offset_y: u32,
    pub state: EyeState,
}

#[derive(Debug)]
pub struct KeyFrame {
    pub frame: u32,
    pub mouth: MouthFrame,
    pub eye: EyeFrame,
}

impl MouthState {
    pub const ALL: [MouthState; 5] = [
        MouthState::Closed,
        MouthState::Small,
        MouthState::Half,
        MouthState::Wide,
        MouthState::Open,
    ];

    pub fn filename_stem(&self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Small => "small",
            Self::Half => "half",
            Self::Wide => "wide",
            Self::Open => "open",
        }
    }
}

impl EyeState {
    pub const ALL: [EyeState; 3] = [EyeState::Open, EyeState::Blink, EyeState::HalfBlink];

    pub fn filename_stem(&self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Blink => "blink",
            Self::HalfBlink => "half_blink",
        }
    }
}
