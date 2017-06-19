extern crate ggez;
extern crate rand;
use ggez::*;
use ggez::graphics::{DrawMode, Point, Color};
use std::time::Duration;


mod piece;
mod point;
mod piecedefs;

use piece::*;

struct MainState {
    piece: Piece
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState { piece: Piece::new() };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        self.piece.draw(ctx);
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, keycode: event::Keycode, _: event::Mod, repeat: bool) {
        if repeat == true {
            return;
        }
        match keycode {
            event::Keycode::Down => self.piece.shift(point::Point { x: 0.0, y: 32.0 }),
            event::Keycode::Left => self.piece.shift(point::Point { x: -32.0, y: 0.0 }),
            event::Keycode::Right => self.piece.shift(point::Point { x: 32.0, y: 0.0 }),
            event::Keycode::Z => self.piece.rotate(3),
            event::Keycode::X => self.piece.rotate(1),
            event::Keycode::C => self.piece.rotate(2),
            event::Keycode::Space => self.piece.hard_drop(),
            _ => return
        }
    }
}

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_height = 704 as u32;
    c.window_width = 320 as u32;
    let ctx = &mut Context::load_from_conf("ggetris", "cn", c).unwrap();
    println!("{:#?}", ctx.filesystem);
    println!("{:?}", ctx.filesystem.read_config());
    println!("{:?}", ctx.filesystem.open("/conf.toml"));
    let bg = Color::new(0.0, 0.0, 0.0, 1.0);
    graphics::set_background_color(ctx, bg);
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
