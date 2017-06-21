/// A state machine to keep track of inputs and
/// some information determining the piece's movements
pub struct InputState {
    pub das: u64,
    pub down_frames: u64,
    pub left: bool,
    pub right: bool,
    pub down: bool,
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            das: 0,
            down_frames: 0,
            left: false,
            right: false,
            down: false,
        }
    }
}
