use ggez::{
    Context,
    GameResult,
};
use crate::gfx::PieceSprites;
use crate::Level;
use super::SCALE;


pub struct Matrix {
    pub state: Vec<Vec<char>>,
    pub cleared: u64,
    pub width: usize,
    pub height: usize, 
    sprites: PieceSprites
}

impl Matrix {
    pub fn new(ctx: &mut Context) -> ggez::GameResult<Matrix> {
        let level = Level::new(ctx, "level1".into())?;
        let state = level.tiles.iter().map(|line| line.iter().map(|tile| {
            match tile {
                Some(_) => 'w',
                None => '0'
            }
        }).collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let mut m = Matrix {
            state: state,
            width: level.width,
            height: level.height,
            cleared: 0,
            sprites: PieceSprites::new(ctx, SCALE)
        };
        m.prepare();
        Ok(m)
    }

    pub fn clear_lines(&mut self) {
        let mut row: usize = 0;
        while row < self.height {
            let mut count = 0;
            for cell in &self.state[row] {
                if *cell != '0' {
                    count += 1;
                }
            }
            if count == self.width {
                self.cleared += 1;
                for temp_row in (0..row).rev() {
                    self.state[temp_row + 1] = self.state[temp_row].clone();
                }
                self.state[0] = vec!['0', '0', '0', '0', '0', '0', '0', '0', '0', '0'];
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
