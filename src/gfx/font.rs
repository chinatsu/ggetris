use ggez::{graphics, mint};

pub struct FontSprites {
    scale: f32,
    spritebatch: graphics::spritebatch::SpriteBatch
}

impl FontSprites {
    pub fn new(ctx: &mut ggez::Context) -> FontSprites {
        let image = graphics::Image::new(ctx, "/gfx/font.png").unwrap();
        let batch = graphics::spritebatch::SpriteBatch::new(image);
        FontSprites {
            scale: 16.0,
            spritebatch: batch
        }
    }

    pub fn clear(&mut self) {
        self.spritebatch.clear();
    }

    pub fn prepare(&mut self, message: String, destination: mint::Point2<f32>) {
        for (i, c) in message.chars().enumerate() {
            let p = graphics::DrawParam::new()
                .src(get_offset(c))
                .dest(mint::Point2{x: destination.x + (i as f32 * self.scale), y: destination.y});
            self.spritebatch.add(p);
        }
    }

    pub fn render(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::draw(ctx, &self.spritebatch, (mint::Point2{x: 0.0, y: 0.0},))
    }
}


fn get_offset(char: char) -> graphics::Rect {
    match char {
        ' ' => get_cell(0, 0),
        '!' => get_cell(0, 1),
        '"' => get_cell(0, 2),
        '#' => get_cell(0, 3),
        '$' => get_cell(0, 4),
        '%' => get_cell(0, 5),
        '&' => get_cell(0, 6),
        '\'' => get_cell(0, 7),
        '(' => get_cell(0, 8),
        ')' => get_cell(0, 9),
        '*' => get_cell(0, 10),
        '+' => get_cell(0, 11),
        ',' => get_cell(0, 12),
        '-' => get_cell(0, 13),
        '.' => get_cell(0, 14),
        '/' => get_cell(0, 15),
        '0' => get_cell(0, 16),
        '1' => get_cell(0, 17),
        '2' => get_cell(0, 18),
        '3' => get_cell(0, 19),
        '4' => get_cell(0, 20),
        '5' => get_cell(0, 21),
        '6' => get_cell(0, 22),
        '7' => get_cell(0, 23),
        '8' => get_cell(0, 24),
        '9' => get_cell(0, 25),
        ':' => get_cell(0, 26),
        ';' => get_cell(0, 27),
        '<' => get_cell(0, 28),
        '=' => get_cell(0, 29),
        '>' => get_cell(0, 30),
        '?' => get_cell(0, 31),
        'a' => get_cell(1, 1),
        'b' => get_cell(1, 2),
        'c' => get_cell(1, 3),
        'd' => get_cell(1, 4),
        'e' => get_cell(1, 5),
        'f' => get_cell(1, 6),
        'g' => get_cell(1, 7),
        'h' => get_cell(1, 8),
        'i' => get_cell(1, 9),
        'j' => get_cell(1, 10),
        'k' => get_cell(1, 11),
        'l' => get_cell(1, 12),
        'm' => get_cell(1, 13),
        'n' => get_cell(1, 14),
        'o' => get_cell(1, 15),
        'p' => get_cell(1, 16),
        'q' => get_cell(1, 17),
        'r' => get_cell(1, 18),
        's' => get_cell(1, 19),
        't' => get_cell(1, 20),
        'u' => get_cell(1, 21),
        'v' => get_cell(1, 22),
        'w' => get_cell(1, 23),
        'x' => get_cell(1, 24),
        'y' => get_cell(1, 25),
        'z' => get_cell(1, 26),
        '[' => get_cell(1, 27),
        '\\' => get_cell(1, 28),
        ']' => get_cell(1, 29),
        '^' => get_cell(1, 30),
        '_' => get_cell(1, 31),
        _ => get_cell(2, 3)
    }
}

fn get_cell(y: usize, x: usize) -> graphics::Rect {
    graphics::Rect::new(x as f32 * 16.0/512.0, y as f32 * 16.0/48.0, 16.0/512.0, 16.0/48.0)
}
