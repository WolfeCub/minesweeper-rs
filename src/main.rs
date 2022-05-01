use bevy::prelude::*;

#[derive(Component)]
struct Tile;

#[derive(Component, Debug)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn new(x: u32, y: u32) -> Position {
        Position { x, y }
    }
}

fn populate_board(mut commands: Commands) {
    commands.spawn().insert(Tile).insert(Position::new(0, 0));
    commands.spawn().insert(Tile).insert(Position::new(7, 11));
    commands.spawn().insert(Tile).insert(Position::new(420, 24));
}

fn board_greeter(query: Query<&Position, With<Tile>>) {
    for p in query.iter() {
        dbg!(p);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(populate_board)
        .add_system(board_greeter)
        .run();
}
