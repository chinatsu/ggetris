use std::time::{Duration, Instant};

pub struct Stats {
    pub time: Instant,
    pub started: u64,

}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            time: Instant::now(),
            started: 0,
        }
    }

    pub fn start(&mut self) {
        self.time = Instant::now();
        self.started = 1
    }

    pub fn stop(&mut self) -> Duration {
        self.started = 2;
        self.time.elapsed()
    }
}
