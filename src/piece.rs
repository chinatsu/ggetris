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
            origin: Point { x: 4.5, y: 1.5 },
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
                    x: (self.origin.x + cell.x) * 32.0,
                    y: (self.origin.y + cell.y) * 32.0,
                    w: 32.0,
                    h: 32.0,
                }
            );
        }
    }

    pub fn get_origin(&mut self) -> Point {
        self.origin
    }

    pub fn shift(&mut self, direction: Point) {
        let new_origin = self.origin + direction;
        if self.can_move_to(new_origin) {
            self.origin = new_origin;
        }
    }

    pub fn hard_drop(&mut self) {
        let mut origin = self.origin;
        while self.can_move_to(origin + Point { x: 0.0, y: 1.0 }) {
            origin = origin + Point { x: 0.0, y: 1.0 };
        }
        self.origin = origin;
    }

    fn can_move_to(&mut self, origin: Point) -> bool {
        for cell in &self.shape[self.orientation] {
            let offset = origin + *cell;
            if offset.x > 10.0 || offset.x < 0.0 {
                return false;
            }
            if offset.y > 22.0 {
                return false;
            }
        }
        true
    }

    pub fn rotate(&mut self, new: usize) {
        self.orientation = (self.orientation + new) % 4;
    }
}
