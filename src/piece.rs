extern crate ggez;
extern crate rand;
use ggez::graphics::{DrawMode, Color};
use ggez::*;
use rand::distributions::{WeightedChoice, IndependentSample};
use point::*;
use piecedefs;


/// Piece struct for storing all the data of a Tetris piece
pub struct Piece {
    pub shape: [[Point; 4]; 4],
    pub origin: Point,
    pub orientation: usize,
    pub id: char,
    pub color: Color,
}

impl Piece {
    /// Creates a new random piece
    pub fn new() -> Piece {
        let mut weights = &mut piecedefs::WEIGHTS;
        let wc = WeightedChoice::new(weights);
        let mut rng = rand::thread_rng();
        let choice = wc.ind_sample(&mut rng);
        let piece = piecedefs::PIECES[choice];
        Piece {
            shape: piece.shape,
            origin: Point { x: 5, y: 2 },
            orientation: 0,
            id: piece.id,
            color: piecedefs::get_color(piece.id)
        }
    }
    /// Draws the piece onto the provided context
    pub fn draw(&mut self, ctx: &mut Context) {
        let _ = graphics::set_color(ctx, self.color);
        for cell in &self.shape[self.orientation] {
            graphics::rectangle(
                ctx,
                DrawMode::Fill,
                graphics::Rect {
                    x: (self.origin.x + cell.x) as f32 * 32.0 - 16.0,
                    y: (self.origin.y + cell.y) as f32 * 32.0 - 16.0,
                    w: 32.0,
                    h: 32.0,
                }
            );
        }
    }
    /// Returns the piece's origin
    pub fn get_origin(&mut self) -> Point {
        self.origin
    }

    /// Moves the piece towards the direction provided
    pub fn shift(&mut self, direction: Point) {
        let new_origin = self.origin + direction;
        if self.can_move_to(new_origin) {
            self.origin = new_origin;
        }
    }

    /// Drops the piece onto the stack
    /// TODO: Commit it to the matrix and spawn a new piece
    pub fn hard_drop(&mut self) {
        self.instant_das(Point { x: 0, y: 1})
    }

    /// A function to instantly move a piece all the way towards
    /// a given direction. Intended for hard drop and
    /// left/right movements when the user has
    /// repeat rate set to "infinite"
    pub fn instant_das(&mut self, direction: Point) {
        let mut origin = self.origin;
        while self.can_move_to(origin + direction) {
            origin = origin +  direction;
        }
        self.origin = origin;
    }

    /// A checking function to see if the piece can move to
    /// the absolute position provided in origin
    fn can_move_to(&mut self, origin: Point) -> bool {
        for cell in &self.shape[self.orientation] {
            let offset = origin + *cell;
            if offset.x > 10 || offset.x <= 0 {
                return false;
            }
            if offset.y > 22 {
                return false;
            }
        }
        true
    }

    /// Rotates the piece to the new orientation
    pub fn rotate(&mut self, new: usize) {
        self.orientation = (self.orientation + new) % 4;
    }
}
