extern crate ggez;
use ggez::graphics::{DrawMode, Color};
use ggez::*;
use point::Point;
use piecedefs;
use matrix::*;
use randomizer::Randomizer;


/// Piece struct for storing all the data of a Tetris piece
pub struct Piece {
    pub shape: [[Point; 4]; 4],
    pub next_shape: piecedefs::Piecedef,
    pub origin: Point,
    pub orientation: usize,
    pub id: char,
    pub color: Color,
    pub randomizer: Randomizer,
}

impl Piece {
    /// Creates a new random piece
    pub fn new() -> Piece {
        let mut randomizer = Randomizer::new();
        let piece = randomizer.next_piece();
        let next_piece = randomizer.next_piece();
        Piece {
            shape: piece.shape,
            next_shape: next_piece,
            origin: Point { x: 5, y: 2 },
            orientation: 0,
            id: piece.id,
            color: piecedefs::get_color(piece.id),
            randomizer: randomizer
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
                    x: (self.origin.x + cell.x) as f32 - 0.5,
                    y: (self.origin.y + cell.y) as f32 - 0.5,
                    w: 1.0,
                    h: 1.0,
                }
            );
        }
    }

    /// Draws the piece onto the provided context
    pub fn draw_next(&mut self, ctx: &mut Context) {
        let _ = graphics::set_color(ctx, piecedefs::get_color(self.next_shape.id));
        for cell in &self.next_shape.shape[0] {
            graphics::rectangle(
                ctx,
                DrawMode::Fill,
                graphics::Rect {
                    x: cell.x as f32 + 12.5,
                    y: cell.y as f32 + 2.0,
                    w: 1.0,
                    h: 1.0,
                }
            );
        }
    }

    pub fn draw_ghost(&mut self, m: &mut Matrix, ctx: &mut Context) {
        let color = Color::new(1.0, 1.0, 1.0, 0.5);
        let real_origin = self.origin;
        self.instant_das(m, Point { x: 0, y: 1 });
        let _ = graphics::set_color(ctx, color);
        for cell in self.shape[self.orientation].iter() {
            graphics::rectangle(
                ctx,
                DrawMode::Fill,
                graphics::Rect {
                    x: (self.origin.x + cell.x) as f32 - 0.5,
                    y: (self.origin.y + cell.y) as f32 - 0.5,
                    w: 1.0,
                    h: 1.0,
                }
            );
        }
        self.origin = real_origin;
    }

    /// Returns the piece's origin
    pub fn get_origin(&mut self) -> Point {
        self.origin
    }

    /// Moves the piece towards the direction provided
    pub fn shift(&mut self, m: &mut Matrix, direction: Point) {
        let new_origin = self.origin + direction;
        let orientation = self.orientation;
        if self.valid_position(m, orientation, new_origin) {
            self.origin = new_origin;
        }
    }

    /// Commit a piece onto the stack, check if any
    /// lines can be cleared, and "make" a new piece
    pub fn lock(&mut self, m: &mut Matrix) {
        for cell in &self.shape[self.orientation] {
            let x = cell.x + self.origin.x - 1;
            let y = cell.y + self.origin.y - 1;
            m.state[y as usize][x as usize] = self.id;
        }
        m.clear_lines();
        self.spawn_piece(m);
    }

    /// Change the shape into a random new one, and reset its
    /// origin and orientation.
    pub fn spawn_piece(&mut self, m: &mut Matrix) {
        let shape = self.next_shape.shape;
        let id = self.next_shape.id;
        let next_shape = self.randomizer.next_piece();
        for cell in &shape[0] {
            let x = 5 + cell.x;
            let y = 2 + cell.y;
            if m.state[y as usize][x as usize] != '0' {
                panic!("Game over!");

            }
        }
        self.shape = shape;
        self.id = id;
        self.next_shape = next_shape;
        self.color = piecedefs::get_color(id);
        self.origin = Point { x: 5, y: 2 };
        self.orientation = 0;
    }

    /// Drops the piece onto the stack
    /// TODO: Commit it to the matrix and spawn a new piece
    pub fn hard_drop(&mut self, m: &mut Matrix) {
        self.instant_das(m, Point { x: 0, y: 1});
        self.lock(m);
    }

    /// A function to instantly move a piece all the way towards
    /// a given direction. Intended for hard drop and
    /// left/right movements when the user has
    /// repeat rate set to "infinite"
    pub fn instant_das(&mut self, m: &mut Matrix, direction: Point) {
        let mut origin = self.origin;
        let orientation = self.orientation;
        while self.valid_position(m, orientation, origin + direction) {
            origin = origin +  direction;
        }
        self.origin = origin;
    }

    /// A checking function to see if the piece can move to
    /// the absolute position provided in origin
    fn valid_position(&mut self, m: &mut Matrix, orientation: usize, origin: Point) -> bool {
        for cell in &self.shape[orientation] {
            let offset = origin + *cell;
            if offset.x > WIDTH as isize || offset.x <= 0 {
                return false;
            }
            if offset.y > HEIGHT as isize || offset.y <= 0 {
                return false;
            }
            if m.state[offset.y as usize - 1][offset.x as usize - 1] != '0' {
                return false;
            }
        }
        true
    }

    /// Rotates the piece to the new orientation.
    /// If the new rotation is blocked by the stack or a wall, we
    /// check an area around the position and place the piece
    /// at the first available spot
    pub fn rotate(&mut self, m: &mut Matrix, new: usize) {
        let new_orientation = (self.orientation + new) % 4;
        let mut new_origin = self.origin;
        if self.valid_position(m, new_orientation, new_origin) {
            self.orientation = new_orientation;
            self.origin = new_origin;
            return;
        }
        let rotate_checks: [Point; 10] = [
            Point{x: -1, y: 0},
            Point{x: 1, y: 0},
            Point{x: 0, y: 1},
            Point{x: 1, y: 1},
            Point{x: -1, y: 1},
            Point{x: -2, y: 0},
            Point{x: 2, y: 0},
            Point{x: 0, y: -1},
            Point{x: -1, y: -1},
            Point{x: 1, y: -1}
        ];
        for check in rotate_checks.iter() {
            new_origin = self.origin + check.clone();
            if self.valid_position(m, new_orientation, new_origin) {
                self.orientation = new_orientation;
                self.origin = new_origin;
                return;
            }
        }
    }
}
