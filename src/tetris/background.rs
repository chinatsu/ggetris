use ggez::{Context, GameResult};
use ggez::graphics::{Image, DrawParam, Rect, WrapMode, draw};
use ggez::mint::{Point2, Vector2};

pub struct Background {
    far: Image,
    middle: Image,
    foreground: Image,
    frame: Image,
    accumulator: i64
}

impl Background {
    pub fn new(ctx: &mut Context) -> GameResult<Background> {
        let mut far = Image::new(ctx, "/gfx/far.png")?;
        let mut middle = Image::new(ctx, "/gfx/sand.png")?;
        let mut fg = Image::new(ctx, "/gfx/foreground-merged.png")?;
        far.set_wrap(WrapMode::Tile, WrapMode::Tile);
        middle.set_wrap(WrapMode::Tile, WrapMode::Tile);
        fg.set_wrap(WrapMode::Tile, WrapMode::Tile);

        Ok(Background {
            far: far,
            middle: middle,
            foreground: fg,
            frame: Image::new(ctx, "/gfx/frame.png")?,
            accumulator: 0
        })
    }


    pub fn render(&mut self, ctx: &mut Context) -> GameResult<()> {

        // the parallax here is dumb af

        draw(
             ctx,
             &self.far,
             DrawParam::new()
                .src(Rect{x: (self.accumulator as f32)/20000.0 % 1.0, y: 0.0, w: 1.0, h: 1.0})
                .dest(Point2{x: 1.0, y: 1.0})
                .scale(Vector2{x: 1.0/32.0, y: 1.0/32.0})
        )?;
        draw(
             ctx,
             &self.middle,
             DrawParam::new()
                .src(Rect{x: (self.accumulator as f32)/10000.0 % 1.0, y: 0.0, w: 1.0, h: 1.0})
                .dest(Point2{x: 1.0, y: 1.0})
                .scale(Vector2{x: 1.0/32.0, y: 1.0/32.0})
        )?;

        draw(
             ctx,
             &self.foreground,
             DrawParam::new()
                .src(Rect{x: (self.accumulator as f32)/8000.0 % 1.0, y: 0.0, w: 1.0, h: 1.0})
                .dest(Point2{x: 1.0, y: 1.0})
                .scale(Vector2{x: 1.0/32.0, y: 1.0/32.0})
        )?;

        draw(
             ctx,
             &self.frame,
             DrawParam::new()
                .dest(Point2{x: 1.0, y: 1.0})
                .scale(Vector2{x: 1.0/32.0, y: 1.0/32.0})
        )?;

        self.accumulator = (1 + self.accumulator) % 20000;
        Ok(())
    }
}
