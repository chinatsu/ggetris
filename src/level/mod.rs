use ggez::filesystem;
use std::io::Read;

#[derive(Debug)]
pub enum Tile {
    Wall,
    Goal,
    Nothing,
}

pub struct Level {
    pub tiles: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
    pub starting_point: (isize, isize)
}

impl Level {
    pub fn new(ctx: &mut ggez::Context, target: String, cave: bool) -> ggez::GameResult<Level> {
        let mut file = filesystem::open(ctx, format!("/level/{}.map", target))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let mut tiles: Vec<Vec<char>> = Vec::new();
        let mut line: Vec<char> = Vec::new();
        let mut x: isize = 0;
        let mut y: isize = 0;
        let mut starting_point: (isize, isize) = (0, 0);
        for c in buffer {
            if c == 0x0a {
                x = 0;
                tiles.push(line);
                line = Vec::new();
                y += 1;
                continue;
            }
            if c == 0x40 {
                starting_point = (x+14, y+14);
            }
            let tile: Tile = c.into();
            line.push(tile.into());
            x += 1;
        }
        if cave {
            tiles = prepare(tiles);
        }
        Ok(Level {
            width: tiles[0].len(),
            height: tiles.len(),
            tiles: tiles,
            starting_point: starting_point
        })
    }
}

impl From<Tile> for char {
    fn from(item: Tile) -> Self {
        match item {
            Tile::Wall => 'w',
            Tile::Nothing => '0',
            Tile::Goal => 's',
        }
    }
}

impl From<u8> for Tile {
    fn from(item: u8) -> Self {
        match item {
            0x23 => Tile::Wall,
            0x67 => Tile::Goal,
            _ => Tile::Nothing
        }
    }
}

fn prepare(tiles: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_tiles = Vec::new(); 
    let pad = 12;
    let width = tiles[0].len();
    let height = tiles.len();
    let real_width = width + pad*2;
    let mapline: Vec<char> = (0..real_width).map(|_| '0').collect();
    for _ in 0..pad {
        new_tiles.push(mapline.clone());
    }

    let mut pad_line = (0..pad).map(|_| '0').collect::<Vec<char>>();

    pad_line.extend((0..width+4).map(|_| 'w').collect::<Vec<char>>());
    pad_line.extend((0..pad).map(|_| '0').collect::<Vec<char>>());

    new_tiles.push(pad_line.clone());
    new_tiles.push(pad_line.clone());

    for y in 0..height {
        let mut mapline: Vec<char> = Vec::new();
        for _ in 0..pad {
            mapline.push('0');
        }
        mapline.push('w');
        mapline.push('w');
        for x in 0..width {
            mapline.push(tiles[y][x]);
        }
        mapline.push('w');
        mapline.push('w');
        for _ in 0..pad {
            mapline.push('0')
        }

        new_tiles.push(mapline);
    }

    new_tiles.push(pad_line.clone());
    new_tiles.push(pad_line);
    for _ in 0..pad {
        new_tiles.push(mapline.clone());
    }
    new_tiles

}