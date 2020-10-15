use ggez::{
    Context,
    GameResult,
};
use crate::sprites::PieceSprites;
use super::SCALE;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 22;

pub struct Matrix {
    pub state: [[char; WIDTH]; HEIGHT],
    pub cleared: u64,
    sprites: PieceSprites
}

impl Matrix {
    pub fn new(ctx: &mut Context) -> Matrix {
        Matrix {
            state: [['0'; WIDTH]; HEIGHT],
            cleared: 0,
            sprites: PieceSprites::new(ctx, SCALE)
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
        self.sprites.clear();
        for y in 0..self.state.len() {
            for x in 0..self.state[y].len() {
                if self.state[y][x] != '0' {
                    self.sprites.prepare(self.state[y][x], x as f32, y as f32);
                }
            }
        }
    }

    pub fn render(&mut self, ctx: &mut Context) -> GameResult {
        self.sprites.render(ctx)
    }
}
