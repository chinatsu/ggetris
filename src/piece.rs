extern crate ggez;
extern crate rand;
use ggez::graphics::{DrawMode, Color};
use ggez::*;
use rand::distributions::{WeightedChoice, IndependentSample};
use point::*;
use piecedefs;

pub struct Piece {
    pub shape: [[Point; 4]; 4],
    pub origin: Point,
    pub orientation: usize,
    pub id: char,
    pub color: Color,
}

impl Piece {
    pub fn new() -> Piece {
        let mut weights = &mut piecedefs::WEIGHTS;
        let wc = WeightedChoice::new(weights);
        let mut rng = rand::thread_rng();
        let choice = wc.ind_sample(&mut rng);
        let piece = piecedefs::PIECES[choice];
        Piece {
            shape: piece.shape,
            origin: Point { x: 144.0, y: 16.0 },
            orientation: 0,
            id: piece.id,
            color: piecedefs::get_color(piece.id)
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let _ = graphics::set_color(ctx, self.color);
        for cell in &self.shape[self.orientation] {
            graphics::rectangle(
                ctx,
                DrawMode::Fill,
                graphics::Rect {
                    x: self.origin.x + cell.x,
                    y: self.origin.y + cell.y,
                    w: 32.0,
                    h: 32.0,
                }
            );
        }
    }

    pub fn get_pos(&mut self) -> Point {
        self.origin
    }

    pub fn shift(&mut self, direction: Point) {
        let new_origin = self.origin + direction;
        if self.can_move(new_origin) {
            self.origin = new_origin;
        }
    }

    pub fn hard_drop(&mut self) {
        let mut o = self.origin;
        while self.can_move(o + Point { x: 0.0, y: 32.0 }) {
            o = o + Point { x: 0.0, y: 32.0 };
        }
        self.origin = o;
    }

    fn can_move(&mut self, origin: Point) -> bool {
        for cell in &self.shape[self.orientation] {
            let offset = origin + *cell;
            if offset.x > 304.0 || offset.x < 16.0 {
                return false;
            }
            if offset.y > 688.0 {
                return false;
            }
        }
        true
    }

    pub fn rotate(&mut self, new: usize) {
        self.orientation = (self.orientation + new) % 4;
    }
}
