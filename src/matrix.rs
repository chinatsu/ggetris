extern crate ggez;
use ggez::graphics::DrawMode;
use ggez::*;
use crate::piecedefs;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 22;

pub struct Matrix {
    pub state: [[char; WIDTH]; HEIGHT],
    pub cleared: u64,
    pub spritebatch: graphics::spritebatch::SpriteBatch
}

impl Matrix {
    pub fn new(ctx: &mut Context) -> Matrix {
        let image = graphics::Image::new(ctx, "/tileset.png").unwrap();
        let batch = graphics::spritebatch::SpriteBatch::new(image);
        Matrix {
            state: [['0'; WIDTH]; HEIGHT],
            cleared: 0,
            spritebatch: batch
        }
    }

    /// Clear fully occupied lines
    pub fn clear_lines(&mut self) {
        let mut row: usize = 0;
        while row <= HEIGHT - 1 {
            let mut count = 0;
            for cell in &self.state[row] {
                if *cell != '0' {
                    count += 1;
                }
            }
            if count == WIDTH {
                self.cleared += 1;
                for temp_row in (0..row).rev() {
                    self.state[temp_row + 1] = self.state[temp_row];
                }
                self.state[0] = ['0'; WIDTH];
            } else {
                row += 1;
            }
        }
        self.prepare();
    }

    fn prepare(&mut self) {
        self.spritebatch.clear();
        for y in 0..self.state.len() {
            for x in 0..self.state[y].len() {
                if self.state[y][x] != '0' {
                    let p = graphics::DrawParam::new()
                        .src(piecedefs::get_offset(self.state[y][x]))
                        .dest(mint::Point2{x: (1+x) as f32, y: (1+y) as f32})
                        .scale(mint::Vector2{x: 1.0/22.0, y: 1.0/22.0});
                    self.spritebatch.add(p);
                }
            }
        }
    }

    pub fn render(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::draw(ctx, &self.spritebatch, (mint::Point2{x: 0.0, y: 0.0},))
    }
}
