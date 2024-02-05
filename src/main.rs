#[macro_use] extern crate lazy_static;

use ggez::{
    ContextBuilder,
    conf::WindowMode,
    conf::WindowSetup,
    event::run,
    GameResult
};
use std::env;
use std::path;

mod config;
mod gfx;
mod state;
mod level;
use state::MainState;
use config::Config;
use level::Level;

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut ctx, event_loop) =
       ContextBuilder::new("ggetris", "chinatsu")
            .window_setup(WindowSetup {
                title: "ggetris".to_string(),
                vsync: true,
                ..Default::default()
                })
            .window_mode(WindowMode {
                height: 22.0 * 32.0,
                width: 16.0 * 32.0,
                resizable: true,
                ..Default::default()
            })
            .add_resource_path(resource_dir)
            .build()?;
            
    let state = MainState::new(&mut ctx)?;
    run(ctx, event_loop, state);
}
