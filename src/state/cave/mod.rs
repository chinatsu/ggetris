use ggez::{event, graphics};
use crate::world::World;
use crate::Player;
use crate::Config;
use crate::point::Point;
use crate::gfx::Background;
use super::inputstate::InputState;

pub struct CaveState {
    world: World,
    player: Player,
    config: Config,
    input: InputState,
    background: Background
}

impl CaveState {
    pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<CaveState> {
        let mut world = World::new(ctx, "cave1".into())?;
        let (x, y) = world.level.starting_point;
        let player = Player::new(ctx, x, y);
        world.prepare(&player);

        let mut background = Background::new(ctx)?;
        background.use_frame(false);
        Ok(CaveState {
            world: world,
            player: player,
            config: Config::new(ctx)?,
            input: InputState::new(),
            background: background,
        })
    }

    pub fn update(&mut self) {
        if self.input.down {
            self.input.down_frames += 1;
            if self.input.down_frames % 1 == 0 {
                self.player.shift(&mut self.world, Point { x: 0, y: 1 })
            }
        }
        if self.input.das > self.config.game.das && self.input.left {
            if self.input.das % 1 == 0 {
                self.player.shift(&mut self.world, Point { x: -1, y: 0 });
            }
            self.input.das += 1;
        } else if self.input.left {
            if self.input.das == 0 {
                self.player.shift(&mut self.world, Point { x: -1, y: 0 });
            }
            self.input.das += 1;
        }
        if self.input.das > self.config.game.das && self.input.right {
            if self.input.das % 1 == 0 {
                self.player.shift(&mut self.world, Point { x: 1, y: 0 });
            }
            self.input.das += 1;
        } else if self.input.right {
            if self.input.das == 0 {
                self.player.shift(&mut self.world, Point { x: 1, y: 0 });
            }
            self.input.das += 1;
        }
    }

    pub fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, graphics::BLACK);
        self.background.render(ctx)?;
        self.world.render(ctx)?;
        self.player.render(ctx)?;

        graphics::present(ctx)
    }

    pub fn key_down_event(&mut self, keycode: event::KeyCode) {
        if keycode == self.config.input.down {
            self.input.down = true
        } else if keycode == self.config.input.harddrop {
        } else if keycode == self.config.input.left {
            self.input.right = false;
            self.input.left = true;
            self.input.das = 0;
        } else if keycode == self.config.input.right {
            self.input.left = false;
            self.input.right = true;
            self.input.das = 0;
        } else if keycode == self.config.input.rotate_cw {
            self.player.rotate(&mut self.world, 3)
        } else if keycode == self.config.input.rotate_ccw {
            self.player.rotate(&mut self.world, 1)
        } else if keycode == self.config.input.flip {
            self.player.rotate(&mut self.world, 2)
        }
    }

    pub fn key_up_event(&mut self, keycode: event::KeyCode) {
        if keycode == self.config.input.down {
            self.input.down = false;
            self.input.down_frames = 0;
        } else if keycode == self.config.input.left {
            self.input.left = false;
        } else if keycode == self.config.input.right {
            self.input.right = false;
        }
    }
}