use ggez::{timer, event};
use super::TetrisState;
use crate::Config;

pub enum State {
    Tetris,
}

pub struct MainState {
    current_state: State,
    tetris_game: TetrisState,
    config: Config
}

impl MainState {
    pub fn new(ctx: &mut ggez::Context) -> ggez::GameResult<MainState> {
        Ok(MainState {
            current_state: State::Tetris,
            tetris_game: TetrisState::new(ctx)?,
            config: Config::new(ctx)?,
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        while timer::check_update_time(ctx, 60) {
            match self.current_state {
                State::Tetris => {
                    self.tetris_game.update();
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        match self.current_state {
            State::Tetris => {
                self.tetris_game.draw(ctx)?;
            }
        }
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut ggez::Context, keycode: event::KeyCode, _: event::KeyMods, _: bool) {
        match self.current_state {
            State::Tetris => {
                self.tetris_game.key_down_event(keycode);
            }
        }

        if keycode == self.config.input.restart {
            self.tetris_game = TetrisState::new(ctx).unwrap()
        }
    }

    fn key_up_event(&mut self, _: &mut ggez::Context, keycode: event::KeyCode, _: event::KeyMods) {
        match self.current_state {
            State::Tetris => {
                self.tetris_game.key_up_event(keycode);
            }
        }
    }
}