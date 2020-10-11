extern crate ggez;
extern crate rand;
use ggez::*;



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
    input: InputState,
    spritebatch: graphics::spritebatch::SpriteBatch,
    fps: f64
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image = graphics::Image::new(ctx, "/tileset.png")?;
        let batch = graphics::spritebatch::SpriteBatch::new(image);
        let s = MainState {
            piece: Piece::new(),
            matrix: Matrix::new(),
            input: InputState::new(),
            spritebatch: batch,
            fps: timer::fps(ctx)
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.fps = timer::fps(ctx);
        print!("{:.2} FPS        \r", self.fps);
        if self.input.down {
            self.input.down_frames += 1;
            if self.input.down_frames % 1 == 0 {
                self.piece.shift(&mut self.matrix, Point { x: 0, y: 1 })
            }
        }
        if self.input.das > 7 && self.input.left {
            self.piece.instant_das(&mut self.matrix, Point { x: -1, y: 0 });
            self.input.das += 1;
        } else if self.input.left {
            if self.input.das == 0 {
                self.piece.shift(&mut self.matrix, Point { x: -1, y: 0 });
            }
            self.input.das += 1;
        }
        if self.input.das > 7 && self.input.right {
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
        graphics::clear(ctx, graphics::Color::new(0.215, 0.231, 0.266, 1.0));

        let line = graphics::Mesh::new_line(ctx, &[mint::Point2{x: 11.0, y: 0.0}, mint::Point2{x: 11.0, y: 23.0}], 0.032, graphics::WHITE)?;
        graphics::draw(ctx, &line, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        self.matrix.prepare_batch(ctx, &mut self.spritebatch)?;
        self.piece.prepare_next(&mut self.spritebatch)?;
        self.piece.prepare_ghost(&mut self.matrix, &mut self.spritebatch)?;
        self.piece.prepare(&mut self.spritebatch)?;

        graphics::draw(ctx, &self.spritebatch, (mint::Point2{x: 0.0, y: 0.0},))?;
        self.spritebatch.clear();

        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, _: &mut Context, keycode: event::KeyCode, _: event::KeyMods, repeat: bool) {
        if repeat == true {
            return;
        }
        match keycode {
            event::KeyCode::Down => self.input.down = true,
            event::KeyCode::Space => self.piece.hard_drop(&mut self.matrix),
            event::KeyCode::Left => {
                self.input.right = false;
                self.input.left = true;
                self.input.das = 0;
            },
            event::KeyCode::Right => {
                self.input.left = false;
                self.input.right = true;
                self.input.das = 0;
            },
            event::KeyCode::Z => self.piece.rotate(&mut self.matrix, 3),
            event::KeyCode::X => self.piece.rotate(&mut self.matrix, 1),
            event::KeyCode::C => self.piece.rotate(&mut self.matrix, 2),
            _ => { }
        }
    }

    fn key_up_event(&mut self, _: &mut Context, keycode: event::KeyCode, _: event::KeyMods) {
        match keycode {
            event::KeyCode::Down => {
                self.input.down = false;
                self.input.down_frames = 0;
            }
            event::KeyCode::Left => {
                self.input.left = false;
            }
            event::KeyCode::Right => {
                self.input.right = false;
            }
            _ => { }
        }

    }
}

pub fn main() {
    let (mut ctx, mut event_loop) =
       ContextBuilder::new("ggetris", "chinatsu")
            .window_setup(conf::WindowSetup {
                title: "ggetris".to_string(),
                vsync: true,
                ..Default::default()
                })
            .window_mode(conf::WindowMode {
                height: 22.0 * 32.0,
                width: 16.0 * 32.0,
                resizable: true,
                ..Default::default()
            })
            .build()
            .unwrap();
    graphics::set_screen_coordinates(&mut ctx, graphics::Rect::new(1.0, 1.0, 16.0, 22.0)).unwrap();
    let state = &mut MainState::new(&mut ctx).unwrap();
    event::run(&mut ctx, &mut event_loop, state).unwrap();
}
