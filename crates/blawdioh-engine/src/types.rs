#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MouthState {
    Closed,
    Small,
    Half,
    Wide,
    Open,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
