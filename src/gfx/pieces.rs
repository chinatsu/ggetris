use ggez::{graphics, mint};

pub struct PieceSprites {
    scale: f32,
    spritebatch: graphics::spritebatch::SpriteBatch
}

impl PieceSprites {
    pub fn new(ctx: &mut ggez::Context, scale: f32) -> PieceSprites {
        let image = graphics::Image::new(ctx, "/gfx/tileset.png").unwrap();
        let batch = graphics::spritebatch::SpriteBatch::new(image);
        PieceSprites {
            scale: scale,
            spritebatch: batch
        }
    }

    pub fn clear(&mut self) {
        self.spritebatch.clear();
    }

    pub fn prepare(&mut self, piece: char, x: f32, y: f32) {
        let p = graphics::DrawParam::new()
            .src(get_offset(piece))
            .dest(mint::Point2{x: self.scale*x, y: self.scale*y});
        self.spritebatch.add(p);
    }

    pub fn render(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::draw(ctx, &self.spritebatch, (mint::Point2{x: 0.0, y: 0.0},))
    }
}


fn get_offset(id: char) -> graphics::Rect {
    match id {
        'z' => get_cell(2),
        'l' => get_cell(2),
        'o' => get_cell(3),
        's' => get_cell(2),
        'i' => get_cell(4),
        'j' => get_cell(4),
        't' => get_cell(4),
        'g' => get_cell(7),
        'w' => get_cell(8),
        '1' => get_cell(1),
        '2' => get_cell(2),
        '3' => get_cell(3),
        '4' => get_cell(4),
        '5' => get_cell(5),
        '6' => get_cell(6),
        '7' => get_cell(7),
        '8' => get_cell(8),
        '9' => get_cell(9),
        _ =>   get_cell(9)
    }
}

fn get_cell(val: u32) -> graphics::Rect {
    graphics::Rect::new(0.0, val as f32 * 32.0 / 384.0, 1.0, 32.0/384.0)
}
