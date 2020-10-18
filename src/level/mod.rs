use ggez::filesystem;
use std::io::Read;

#[derive(Debug)]
pub enum Tile {
    Wall,
}

pub struct Level {
    pub tiles: Vec<Vec<Option<Tile>>>,
    pub width: usize,
    pub height: usize
}

impl Level {
    pub fn new(ctx: &mut ggez::Context, target: String) -> ggez::GameResult<Level> {
        let mut file = filesystem::open(ctx, format!("/level/{}.map", target))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let mut tiles: Vec<Vec<Option<Tile>>> = Vec::new();
        let mut line: Vec<Option<Tile>> = Vec::new();
        for c in buffer {
            if c == 0x0a {
                tiles.push(line);
                line = Vec::new();
                continue;
            }
            if c == 0x23 {
                line.push(Some(Tile::Wall));
            } else {
                line.push(None);
            }
        }
        Ok(Level {
            width: tiles[0].len(),
            height: tiles.len(),
            tiles: tiles,
        })
    }
}
