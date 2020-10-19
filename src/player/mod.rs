use crate::gfx::PieceSprites;
use crate::world::World;
use crate::point::Point;
use crate::definitions::pieces;

pub struct Player {
    pub origin: Point,
    piece: pieces::Piecedef,
    orientation: usize,
    sprites: PieceSprites
}

impl Player {
    pub fn new(ctx: &mut ggez::Context, x: isize, y: isize) -> Player {
        Player {
            origin: Point{x: x, y: y},
            piece: pieces::S,
            orientation: 0,
            sprites: PieceSprites::new(ctx, 32.0)
        }
    }

    pub fn shift(&mut self, m: &mut World, direction: Point) {
        let new_origin = self.origin + direction;
        if self.valid_position(m, self.orientation, new_origin) {
            self.origin = new_origin;
            m.prepare(&self);
        }
    }

    fn valid_position(&mut self, m: &mut World, orientation: usize, origin: Point) -> bool {
        for cell in &self.piece.shape[orientation] {
            let offset = origin + *cell;
            if offset.x >= m.level.width as isize || offset.x < 0 {
                return false;
            }
            if offset.y >= m.level.height as isize || offset.y < 0 {
                return false;
            }
            if m.level.tiles[offset.y as usize][offset.x as usize] != '0' {
                return false;
            }
        }
        true
    }

    pub fn rotate(&mut self, m: &mut World, new: usize) {
        let new_orientation = (self.orientation + new) % 4;
        let mut new_origin = self.origin;
        if self.valid_position(m, new_orientation, new_origin) {
            self.orientation = new_orientation;
            self.origin = new_origin;
            m.prepare(&self);
            return;
        }
        let rotate_checks: Vec<Point> = vec![
            Point{x: -1, y: 0},
            Point{x: 1, y: 0},
            Point{x: 0, y: 1},
            Point{x: 1, y: 1},
            Point{x: -1, y: 1},
            Point{x: -2, y: 0},
            Point{x: 2, y: 0},
            Point{x: 1, y: 2},
            Point{x: -1, y: 2},
            Point{x: 0, y: -1},
            Point{x: -1, y: -1},
            Point{x: 1, y: -1}
        ];
        for check in rotate_checks.iter() {
            new_origin = self.origin + check.clone();
            if self.valid_position(m, new_orientation, new_origin) {
                self.orientation = new_orientation;
                self.origin = new_origin;
                m.prepare(&self);
                return;
            }
        }
    }

    fn prepare(&mut self) {
        for cell in &self.piece.shape[self.orientation] {
            self.sprites.prepare(self.piece.id, (10 + cell.x) as f32, (10 + cell.y) as f32);
        }
    }


    pub fn render(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.sprites.clear();
        self.prepare();
        self.sprites.render(ctx)
    }
}