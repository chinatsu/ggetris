use ggez::{Context, GameResult};
use ggez::graphics::{Image, DrawParam, Rect, WrapMode, draw};
use ggez::mint::{Point2, Vector2};
use std::path::Path;

pub struct Background {
    far: ParallaxImage,
    middle: ParallaxImage,
    foreground: ParallaxImage,
    frame: Image,
}

impl Background {
    pub fn new(ctx: &mut Context) -> GameResult<Background> {
        Ok(Background {
            far: ParallaxImage::new(ctx, "/gfx/far.png", 0.00003)?,
            middle: ParallaxImage::new(ctx, "/gfx/sand.png", 0.00007)?,
            foreground: ParallaxImage::new(ctx, "/gfx/foreground-merged.png", 0.00016)?,
            frame: Image::new(ctx, "/gfx/frame.png")?
        })
    }


    pub fn render(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.far.draw(ctx)?;
        self.middle.draw(ctx)?;
        self.foreground.draw(ctx)?;

        draw(
             ctx,
             &self.frame,
             DrawParam::new()
                .dest(Point2{x: 0.0, y: 0.0})
                .scale(Vector2{x: 1.0, y: 1.0})
        )?;

        Ok(())
    }
}

pub struct ParallaxImage {
    image: Image,
    acc: f32,
    speed: f32
}

impl ParallaxImage {
    fn new<P: AsRef<Path>>(ctx: &mut Context, path: P, speed: f32) -> GameResult<ParallaxImage> {
        let mut img = Image::new(ctx, path)?;
        img.set_wrap(WrapMode::Tile, WrapMode::Tile);
        Ok(ParallaxImage {
            image: img,
            acc: 0.0,
            speed: speed
        })
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        draw(
             ctx,
             &self.image,
             DrawParam::new()
                .src(Rect{x: self.acc, y: 0.0, w: 1.0, h: 1.0})
        )?;
        self.acc = (self.acc + self.speed) % 1.0;
        Ok(())
    }
}