extern crate ggez;
use ggez::graphics::DrawMode;
use ggez::*;
use piecedefs;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 22;

pub struct Matrix {
    pub state: [[char; WIDTH]; HEIGHT],
    pub cleared: u64
}

impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            state: [['0'; WIDTH]; HEIGHT],
            cleared: 0
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
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        for y in 0..self.state.len() {
            for x in 0..self.state[y].len() {
                if self.state[y][x] != '0' {
                    let color = piecedefs::get_color(self.state[y][x]);
                    let _ = graphics::set_color(ctx, color);
                    graphics::rectangle(
                        ctx,
                        DrawMode::Fill,
                        graphics::Rect {
                            x: (1 + x) as f32 - 0.5,
                            y: (1 + y) as f32 - 0.5,
                            w: 1.0,
                            h: 1.0,
                        }
                    );
                }
            }
        }
    }
}
