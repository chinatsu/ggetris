pub struct InputState {
    pub das_right: u64,
    pub das_left: u64,
    pub down_frames: u64,
    pub left: bool,
    pub right: bool,
    pub down: bool,
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            das_right: 0,
            das_left: 0,
            down_frames: 0,
            left: false,
            right: false,
            down: false,
        }
    }
}
