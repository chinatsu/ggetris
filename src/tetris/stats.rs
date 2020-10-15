use std::time::{Duration, Instant};
use ggez::mint;
use crate::sprites::FontSprites;
use super::matrix::Matrix;

pub struct Stats {
    pub instant: Instant,
    pub time: Duration,
    pub started: u64,
    pub sprites: FontSprites

}

impl Stats {
    pub fn new(ctx: &mut ggez::Context) -> Stats {
        Stats {
            instant: Instant::now(),
            time: Duration::from_millis(0),
            started: 0,
            sprites: FontSprites::new(ctx)
        }
    }

    pub fn start(&mut self) {
        self.instant = Instant::now();
        self.started = 1
    }

    pub fn stop(&mut self) {
        self.started = 2;
        self.update();
    }

    fn update(&mut self) {
        self.time = self.instant.elapsed();
    }

    pub fn render(&mut self, ctx: &mut ggez::Context, m: &Matrix) -> ggez::GameResult {
        if self.started == 1 {
            self.update();
        }
        self.sprites.clear();
        self.sprites.prepare(String::from("stats:"), mint::Point2{x: 0.0, y: 0.0});
        self.sprites.prepare(format!("lines:{}", m.cleared), mint::Point2{x: 0.0, y: 32.0});
        if self.started > 0 {
            self.sprites.prepare(format!("40l:{}.{:.2}", self.time.as_secs(), self.time.subsec_millis()), mint::Point2{x: 0.0, y: 48.0});
        }
        self.sprites.render(ctx)
    }
}
