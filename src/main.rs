#[macro_use] extern crate lazy_static;

use ggez::{
    ContextBuilder,
    conf::WindowMode,
    conf::WindowSetup,
    event::run,
    graphics::Rect,
    graphics::set_screen_coordinates
};
use std::env;
use std::path;

mod tetris;
mod input;
use tetris::TetrisState;
use input::Config;

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
                height: 22.0 * 32.0,
                width: 16.0 * 32.0,
                resizable: true,
                ..Default::default()
            })
            .add_resource_path(resource_dir)
            .build()
            .unwrap();

    // TODO: sort this crap out and use actual coordinates >:(
    set_screen_coordinates(&mut ctx, Rect::new(1.0, 1.0, 16.0, 22.0)).unwrap();
    let state = &mut TetrisState::new(&mut ctx).unwrap();
    run(&mut ctx, &mut event_loop, state).unwrap();
}
