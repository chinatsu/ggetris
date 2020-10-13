use ggez::{GameResult, Context};
use ggez::graphics::{
    DrawParam,
    Image,
    draw,
    mint::Point2,
    mint::Vector2,
    spritebatch::SpriteBatch,

};

use super::point::Point;
use super::piecedefs::{Piecedef, get_offset};
use super::matrix::{Matrix, HEIGHT, WIDTH};
use super::stats::Stats;
use super::randomizer::Randomizer;


pub struct Piece {
    pub piece: Piecedef,
    pub next_piece: Piecedef,
    pub origin: Point,
    pub orientation: usize,
    pub randomizer: Randomizer,
    pub spritebatch: SpriteBatch
}

impl Piece {
    pub fn new(ctx: &mut Context) -> Option<Piece> {
        let mut randomizer = Randomizer::new();
        let image = Image::new(ctx, "/gfx/tileset.png").unwrap();
        let batch = SpriteBatch::new(image);

        Some(Piece {
            piece: randomizer.next_piece()?,
            next_piece: randomizer.next_piece()?,
            origin: Point { x: 5, y: 2 },
            orientation: 0,
            randomizer: randomizer,
            spritebatch: batch
        })
    }

    pub fn shift(&mut self, m: &mut Matrix, direction: Point) {
        let new_origin = self.origin + direction;
        let orientation = self.orientation;
        if self.valid_position(m, orientation, new_origin) {
            self.origin = new_origin;
        }
    }

    pub fn lock(&mut self, m: &mut Matrix) {
        for cell in &self.piece.shape[self.orientation] {
            let x = cell.x + self.origin.x - 1;
            let y = cell.y + self.origin.y - 1;
            m.state[y as usize][x as usize] = self.piece.id;
        }
        m.clear_lines();
        self.spawn_piece(m);
    }

    pub fn spawn_piece(&mut self, m: &mut Matrix) -> Option<()> {
        self.piece = self.next_piece;
        self.next_piece = self.randomizer.next_piece()?;
        self.origin = Point { x: 5, y: 2 };
        self.orientation = 0;
        if !self.valid_position(m, self.orientation, self.origin) {
            panic!("Game over!");
        }
        Some(())
    }

    pub fn hard_drop(&mut self, m: &mut Matrix, stats: &mut Stats) {
        if stats.started == 0 {
            stats.start();
        }
        self.instant_das(m, Point { x: 0, y: 1});
        self.lock(m);
        if stats.started == 1 && m.cleared >= 40 {
            let time = stats.stop();
            println!("{}.{} seconds", time.as_secs(), time.subsec_nanos());
        }
    }

    pub fn instant_das(&mut self, m: &mut Matrix, direction: Point) {
        let mut origin = self.origin;
        let orientation = self.orientation;
        while self.valid_position(m, orientation, origin + direction) {
            origin = origin + direction;
        }
        self.origin = origin;
    }

    fn valid_position(&mut self, m: &mut Matrix, orientation: usize, origin: Point) -> bool {
        for cell in &self.piece.shape[orientation] {
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

    pub fn rotate(&mut self, m: &mut Matrix, new: usize) {
        let new_orientation = (self.orientation + new) % 4;
        let mut new_origin = self.origin;
        if self.valid_position(m, new_orientation, new_origin) {
            self.orientation = new_orientation;
            self.origin = new_origin;
            return;
        }
        let rotate_checks: Vec<Point> = vec![
            Point{x: -1, y: 0},
            Point{x: 1, y: 0},
            Point{x: 0, y: 1},
            Point{x: 0, y: -1},
            Point{x: 1, y: 1},
            Point{x: -1, y: 1},
            Point{x: -2, y: 0},
            Point{x: 2, y: 0},
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

    pub fn render(&mut self, ctx: &mut Context, matrix: &mut Matrix) -> GameResult<()> {
        self.prepare_next()?;
        self.prepare_ghost(matrix)?;
        self.prepare()?;
        draw(ctx, &self.spritebatch, (Point2{x: 0.0, y: 0.0},))?;
        self.spritebatch.clear();
        Ok(())
    }

    fn prepare(&mut self) -> GameResult<()> {
        for cell in &self.piece.shape[self.orientation] {
            let p = DrawParam::new()
                .src(get_offset(self.piece.id))
                .dest(Point2{x: (self.origin.x + cell.x) as f32, y: (self.origin.y + cell.y) as f32})
                .scale(Vector2{x: 1.0/32.0, y: 1.0/32.0});
            self.spritebatch.add(p);

        }
        Ok(())
    }

    fn prepare_next(&mut self) -> GameResult<()> {
        for cell in &self.next_piece.shape[0] {
            let p = DrawParam::new()
                .src(get_offset(self.next_piece.id))
                .dest(Point2{x: (13+cell.x) as f32, y: (4+cell.y) as f32})
                .scale(Vector2{x: 1.0/32.0, y: 1.0/32.0});
            self.spritebatch.add(p);
        }
        Ok(())
    }

    fn prepare_ghost(&mut self, m: &mut Matrix) -> GameResult<()> {
        let real_origin = self.origin;
        self.instant_das(m, Point { x: 0, y: 1 });
        for cell in self.piece.shape[self.orientation].iter() {
            let p = DrawParam::new()
                .src(get_offset('g'))
                .dest(Point2{x: (self.origin.x + cell.x) as f32, y: (self.origin.y + cell.y) as f32})
                .scale(Vector2{x: 1.0/32.0, y: 1.0/32.0});
            self.spritebatch.add(p);
        }
        self.origin = real_origin;
        Ok(())
    }
}
