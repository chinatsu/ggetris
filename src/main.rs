extern crate ggez;
extern crate rand;
use ggez::*;
use ggez::graphics::{DrawMode, Color};
use std::time::Duration;


mod piece;
mod point;
mod piecedefs;
mod inputstate;
mod matrix;

use piece::Piece;
use matrix::Matrix;
use point::Point;
use inputstate::InputState;

struct MainState {
    piece: Piece,
    matrix: Matrix,
    input: InputState
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            piece: Piece::new(),
            matrix: Matrix::new(),
            input: InputState::new()
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        if self.input.down {
            self.input.down_frames += 1;
            if self.input.down_frames % 2 == 0 {
                self.piece.shift(&mut self.matrix, Point { x: 0, y: 1 })
            }
        }
        if self.input.das_left > 10 && self.input.left {
            self.piece.instant_das(&mut self.matrix, Point { x: -1, y: 0 });
        } else if self.input.left {
            if self.input.das_left == 0 {
                self.piece.shift(&mut self.matrix, Point { x: -1, y: 0 });
            }
            self.input.das_left += 1;
        }
        if self.input.das_right > 10 && self.input.right {
            self.piece.instant_das(&mut self.matrix, Point { x: 1, y: 0 });
        } else if self.input.right {
            if self.input.das_right == 0 {
                self.piece.shift(&mut self.matrix, Point { x: 1, y: 0 });
            }
            self.input.das_right += 1;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        self.matrix.draw(ctx);
        self.piece.draw(ctx);
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, keycode: event::Keycode, _: event::Mod, repeat: bool) {
        if repeat == true {
            return;
        }
        match keycode {
            event::Keycode::Down => self.input.down = true,
            event::Keycode::Space => self.piece.hard_drop(&mut self.matrix),
            event::Keycode::Left => {
                self.input.right = false;
                self.input.left = true;
            },
            event::Keycode::Right => {
                self.input.left = false;
                self.input.right = true;
            },
            event::Keycode::Z => self.piece.rotate(3),
            event::Keycode::X => self.piece.rotate(1),
            event::Keycode::C => self.piece.rotate(2),
            _ => { }
        }
    }

    fn key_up_event(&mut self, keycode: event::Keycode, _: event::Mod, repeat: bool) {
        if repeat == true {
            return;
        }

        match keycode {
            event::Keycode::Down => {
                self.input.down = false;
                self.input.down_frames = 0;
            }
            event::Keycode::Left => {
                self.input.left = false;
                self.input.das_left = 0;
            }
            event::Keycode::Right => {
                self.input.right = false;
                self.input.das_right = 0;
            }
            _ => { }
        }

    }
}

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_height = 704 as u32;
    c.window_width = 320 as u32;
    let ctx = &mut Context::load_from_conf("ggetris", "cn", c).unwrap();
    let bg = Color::new(0.0, 0.0, 0.0, 1.0);
    graphics::set_background_color(ctx, bg);
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
