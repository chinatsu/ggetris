use ggez::{
    Context,
    GameResult,
    mint::Point2,
    graphics::Image,
    graphics::DrawParam,
    graphics::draw,
    graphics::spritebatch::SpriteBatch,
};
use super::piecedefs::get_offset;
use super::SCALE;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 22;

pub struct Matrix {
    pub state: [[char; WIDTH]; HEIGHT],
    pub cleared: u64,
    spritebatch: SpriteBatch
}

impl Matrix {
    pub fn new(ctx: &mut Context) -> Matrix {
        let image = Image::new(ctx, "/gfx/tileset.png").unwrap();
        let batch = SpriteBatch::new(image);
        Matrix {
            state: [['0'; WIDTH]; HEIGHT],
            cleared: 0,
            spritebatch: batch
        }
    }

    pub fn clear_lines(&mut self) {
        let mut row: usize = 0;
        while row < HEIGHT {
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
                    let p = DrawParam::new()
                        .src(get_offset(self.state[y][x]))
                        .dest(Point2{x: SCALE*x as f32, y: SCALE*y as f32});
                    self.spritebatch.add(p);
                }
            }
        }
    }

    pub fn render(&mut self, ctx: &mut Context) -> GameResult {
        draw(ctx, &self.spritebatch, (Point2{x: 0.0, y: 0.0},))
    }
}
