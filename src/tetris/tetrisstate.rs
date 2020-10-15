use ggez::{
    Context,
    GameResult,
    graphics,
    event::EventHandler,
    event::KeyCode,
    event::KeyMods,
    timer,
};

use super::piece::Piece;
use super::matrix::Matrix;
use super::point::Point;
use super::inputstate::InputState;
use super::stats::Stats;
use super::background::Background;
use crate::Config;

pub struct TetrisState {
    config: Config,
    piece: Piece,
    matrix: Matrix,
    input: InputState,
    stats: Stats,
    background: Background
}

impl TetrisState {
    pub fn new(ctx: &mut Context) -> GameResult<TetrisState> {
        Ok(TetrisState {
            config: Config::new(ctx)?,
            piece: Piece::new(ctx).unwrap(),
            matrix: Matrix::new(ctx),
            input: InputState::new(),
            stats: Stats::new(ctx),
            background: Background::new(ctx)?
        })
    }
}

impl EventHandler for TetrisState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, 60) {
            if self.input.down {
                self.input.down_frames += 1;
                if self.input.down_frames % 1 == 0 {
                    self.piece.shift(&mut self.matrix, Point { x: 0, y: 1 })
                }
            }
            if self.input.das > self.config.game.das && self.input.left {
                self.piece.instant_das(&mut self.matrix, Point { x: -1, y: 0 });
                self.input.das += 1;
            } else if self.input.left {
                if self.input.das == 0 {
                    self.piece.shift(&mut self.matrix, Point { x: -1, y: 0 });
                }
                self.input.das += 1;
            }
            if self.input.das > self.config.game.das && self.input.right {
                self.piece.instant_das(&mut self.matrix, Point { x: 1, y: 0 });
                self.input.das += 1;
            } else if self.input.right {
                if self.input.das == 0 {
                    self.piece.shift(&mut self.matrix, Point { x: 1, y: 0 });
                }
                self.input.das += 1;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        self.background.render(ctx)?;
        self.matrix.render(ctx)?;
        self.piece.render(ctx, &mut self.matrix)?;
        self.stats.render(ctx, &self.matrix)?;
        graphics::present(ctx)?;

        print!("Framerate: {:.2}    \r", timer::fps(ctx));
        Ok(())
    }

    fn key_down_event(&mut self, _: &mut Context, keycode: KeyCode, _: KeyMods, repeat: bool) {
        if repeat == true {
            return;
        }

        if keycode == self.config.input.down {
            self.input.down = true
        } else if keycode == self.config.input.harddrop {
            self.piece.hard_drop(&mut self.matrix, &mut self.stats)
        } else if keycode == self.config.input.left {
            self.input.right = false;
            self.input.left = true;
            self.input.das = 0;
        } else if keycode == self.config.input.right {
            self.input.left = false;
            self.input.right = true;
            self.input.das = 0;
        } else if keycode == self.config.input.rotate_cw {
            self.piece.rotate(&mut self.matrix, 3)
        } else if keycode == self.config.input.rotate_ccw {
            self.piece.rotate(&mut self.matrix, 1)
        } else if keycode == self.config.input.flip {
            self.piece.rotate(&mut self.matrix, 2)
        }
    }

    fn key_up_event(&mut self, _: &mut Context, keycode: KeyCode, _: KeyMods) {
        if keycode == self.config.input.down {
            self.input.down = false;
            self.input.down_frames = 0;
        } else if keycode == self.config.input.left {
            self.input.left = false;
        } else if keycode == self.config.input.right {
            self.input.right = false;
        }
    }
}
