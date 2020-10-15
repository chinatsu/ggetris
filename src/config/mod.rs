use ggez::{GameResult, Context, filesystem};
use ggez::event::KeyCode;
use serde::{Deserialize};
use std::io::Read;

mod input;

#[derive(Deserialize)]
struct TomlConfig {
    input: InputTomlConfig,
    game: GameTomlConfig
}

#[derive(Deserialize)]
struct InputTomlConfig {
    left: String,
    right: String,
    down: String,
    harddrop: String,
    rotate_cw: String,
    rotate_ccw: String,
    flip: String
}

#[derive(Deserialize)]
pub struct GameTomlConfig {
    pub das: u64
}

pub struct Config {
    pub input: InputConfig,
    pub game: GameTomlConfig
}

pub struct InputConfig {
    pub left: KeyCode,
    pub right: KeyCode,
    pub down: KeyCode,
    pub harddrop: KeyCode,
    pub rotate_cw: KeyCode,
    pub rotate_ccw: KeyCode,
    pub flip: KeyCode
}

impl Config {
    pub fn new(ctx: &mut Context) -> GameResult<Config> {
        let mut file = filesystem::open(ctx, "/config.toml")?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let config: TomlConfig = toml::from_slice(&buffer).unwrap();

        Ok(Config {
            input: InputConfig {
                left: input::get_keycode(config.input.left),
                right: input::get_keycode(config.input.right),
                down: input::get_keycode(config.input.down),
                harddrop: input::get_keycode(config.input.harddrop),
                rotate_cw: input::get_keycode(config.input.rotate_cw),
                rotate_ccw: input::get_keycode(config.input.rotate_ccw),
                flip: input::get_keycode(config.input.flip),
            },
            game: config.game
        })
    }
}
