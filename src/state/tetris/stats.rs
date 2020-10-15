use std::time::{Duration, Instant};
use ggez::{mint, timer};
use crate::gfx::FontSprites;
use super::matrix::Matrix;

#[derive(PartialEq)]
pub enum TimerStatus {
    Uninitialized,
    Started,
    Stopped
}

pub struct Stats {
    pub instant: Instant,
    pub time: Duration,
    pub started: TimerStatus,
    pub sprites: FontSprites

}

impl Stats {
    pub fn new(ctx: &mut ggez::Context) -> Stats {
        Stats {
            instant: Instant::now(),
            time: Duration::from_millis(0),
            started: TimerStatus::Uninitialized,
            sprites: FontSprites::new(ctx)
        }
    }

    pub fn start(&mut self) {
        self.instant = Instant::now();
        self.started = TimerStatus::Started;
    }

    pub fn stop(&mut self) {
        self.started = TimerStatus::Stopped;
        self.update();
    }

    fn update(&mut self) {
        self.time = self.instant.elapsed();
    }

    pub fn render(&mut self, ctx: &mut ggez::Context, m: &Matrix) -> ggez::GameResult {
        if self.started == TimerStatus::Started {
            self.update();
        }
        self.sprites.clear();
        self.sprites.prepare(String::from("stats:"), mint::Point2{x: 32.0*10.5, y: 32.0*5.0});
        self.sprites.prepare(format!("lines:{}", m.cleared), mint::Point2{x: 32.0*10.5, y: 32.0*6.0});
        if self.started != TimerStatus::Uninitialized {
            self.sprites.prepare(format!("40l:{}.{:.2}", self.time.as_secs(), self.time.subsec_millis()), mint::Point2{x: 32.0*10.5, y: 32.0*7.0});
        }
        self.sprites.prepare(format!("fps:{:.2}", timer::fps(ctx)), mint::Point2{x: 32.0*10.5, y: 32.0*21.0});
        self.sprites.render(ctx)
    }
}
