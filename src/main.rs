#[macro_use] extern crate lazy_static;

use ggez::{
    ContextBuilder,
    conf::WindowMode,
    conf::WindowSetup,
    event::run
};
use std::env;
use std::path;

mod config;
mod gfx;
mod state;
mod level;
mod world;
mod player;
mod point;
mod definitions;
use state::MainState;
use config::Config;
use level::Level;
use player::Player;

pub fn main() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut ctx, mut event_loop) =
       ContextBuilder::new("ggetris", "chinatsu")
            .window_setup(WindowSetup {
                title: "ggetris".to_string(),
                vsync: true,
                ..Default::default()
                })
            .window_mode(WindowMode {
                height: 21.0 * 32.0,
                width: 21.0 * 32.0,
                resizable: true,
                ..Default::default()
            })
            .add_resource_path(resource_dir)
            .build()
            .unwrap();

    let state = &mut MainState::new(&mut ctx).unwrap();
    run(&mut ctx, &mut event_loop, state).unwrap();
}
