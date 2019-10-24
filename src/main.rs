extern crate ggez;
extern crate rand;
//extern crate flame;
use ggez::*;
use ggez::graphics::Color;
use std::time::Duration;
//use std::fs::File;



mod piece;
mod point;
mod piecedefs;
mod inputstate;
mod matrix;
mod randomizer;

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
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        //print!("{:.2} FPS        \r", timer::get_fps(_ctx));
        if self.input.down {
            self.input.down_frames += 1;
            if self.input.down_frames % 1 == 0 {
                self.piece.shift(&mut self.matrix, Point { x: 0, y: 1 })
            }
        }
        if self.input.das > 8 && self.input.left {
            self.piece.instant_das(&mut self.matrix, Point { x: -1, y: 0 });
            self.input.das += 1;
        } else if self.input.left {
            if self.input.das == 0 {
                self.piece.shift(&mut self.matrix, Point { x: -1, y: 0 });
            }
            self.input.das += 1;
        }
        if self.input.das > 8 && self.input.right {
            self.piece.instant_das(&mut self.matrix, Point { x: 1, y: 0 });
            self.input.das += 1;
        } else if self.input.right {
            if self.input.das == 0 {
                self.piece.shift(&mut self.matrix, Point { x: 1, y: 0 });
            }
            self.input.das += 1;
        }
        //timer::sleep_until_next_frame(_ctx, 120);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        //flame::start("draw matrix");
        self.matrix.draw(ctx);

        graphics::line(ctx, &[graphics::Point2::new(10.0, 0.0), graphics::Point2::new(10.0, 22.0)], 0.032);
        //flame::end("draw matrix");
        //flame::start("draw ghost");
        self.piece.draw_ghost(&mut self.matrix, ctx);
        self.piece.draw_next(ctx);
        //flame::end("draw ghost");
        //flame::start("draw piece");
        self.piece.draw(ctx);
        //flame::end("draw piece");
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, _: &mut Context, keycode: event::Keycode, _: event::Mod, repeat: bool) {
        if repeat == true {
            return;
        }
        match keycode {
            event::Keycode::Down => self.input.down = true,
            event::Keycode::Space => self.piece.hard_drop(&mut self.matrix),
            event::Keycode::Left => {
                self.input.right = false;
                self.input.left = true;
                self.input.das = 0;
            },
            event::Keycode::Right => {
                self.input.left = false;
                self.input.right = true;
                self.input.das = 0;
            },
            event::Keycode::Z => self.piece.rotate(&mut self.matrix, 3),
            event::Keycode::X => self.piece.rotate(&mut self.matrix, 1),
            event::Keycode::C => self.piece.rotate(&mut self.matrix, 2),
            _ => { }
        }
    }

    fn key_up_event(&mut self, _: &mut Context, keycode: event::Keycode, _: event::Mod, repeat: bool) {
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
            }
            event::Keycode::Right => {
                self.input.right = false;
            }
            _ => { }
        }

    }
}

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_mode.height = 22 * 32 as u32;
    c.window_mode.width = 16 * 32 as u32;
    //c.vsync = false;
    let ctx = &mut Context::load_from_conf("ggetris", "cn", c).unwrap();
    let bg = Color::new(0.0, 0.0, 0.0, 1.0);
    graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, 16.0, 22.0));
    graphics::set_background_color(ctx, bg);
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
    //flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
}
