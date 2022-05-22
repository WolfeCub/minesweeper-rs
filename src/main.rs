use bevy::{prelude::*, window::PresentMode};

mod minesweeper_plugin;
use minesweeper_plugin::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "minesweeper.rs".to_string(),
            width: 500.,
            height: 300.,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins)
        .add_plugin(MinesweeperPlugin)
        .run();
}

