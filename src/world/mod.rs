use crate::{Level, Player, gfx::PieceSprites};

pub struct World {
    pub level: Level,
    sprites: PieceSprites
}

impl World {
    pub fn new(ctx: &mut ggez::Context, level: String) -> ggez::GameResult<World> {
        Ok(World{
            level: Level::new(ctx, level.into(), true)?,
            sprites: PieceSprites::new(ctx, 32.0)
        })
    }

    pub fn view(&mut self, player: &Player) -> Vec<Vec<char>> {
        let mut subsection: Vec<Vec<char>> = Vec::new();
        for line in self.level.tiles[player.origin.y as usize-10..player.origin.y as usize+11].iter() {
            subsection.push(line[player.origin.x as usize-10..player.origin.x as usize+11].to_vec())
        }
        subsection
    }

    pub fn render(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.sprites.render(ctx)
    }

    pub fn prepare(&mut self, player: &Player) {
        let state = self.view(player);
        self.sprites.clear();
        for y in 0..state.len() {
            for x in 0..state[y].len() {
                if state[y][x] != '0' {
                    self.sprites.prepare(state[y][x], x as f32, y as f32);
                }
            }
        }
    }
}