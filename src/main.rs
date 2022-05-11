use bevy::{prelude::*, window::PresentMode};

mod board;
use board::*;

mod position;
use position::*;

mod tile;
use tile::*;


#[derive(Component)]
struct ToUncover;

struct GameState {
    pub complete: bool,
}

impl GameState {
    fn new() -> Self {
        Self {
            complete: false,
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    /* TODO: Not hardcoded */
    let mut board = Board::new(20, 20);
    for _ in 0..30 {
        board.add_bomb();
    }

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let font = asset_server.load("FiraMono-Regular.ttf");

    let sprite_center = board.sprite_size / 2.;
    let max_width = (board.width as f32) * board.sprite_size / 2.;
    let max_height = (board.height as f32) * board.sprite_size / 2.;

    for col in 0..board.height {
        let x = -max_width + sprite_center + (col as f32 * board.sprite_size);

        for row in 0..board.width {
            let y = max_height - sprite_center - (row as f32 * board.sprite_size);

            let covered_entity = commands.spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(x, y, 1.),
                sprite: Sprite {
                    color: Color::rgb(0.5, 0.5, 0.5),
                    custom_size: Some(Vec2::new(board.sprite_size - 2., board.sprite_size - 2.)),
                    ..default()
                },
                ..default()
            })
            .insert(Position::new(col, row))
            .id();

            board.add_covered(row, col, covered_entity);

            match board.get(row, col) {
                Some(Tile::Bomb) => {
                    commands.spawn_bundle(SpriteBundle {
                        texture: asset_server.load("bomb.png"),
                        transform: Transform::from_xyz(x, y, 0.),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(board.sprite_size - 2., board.sprite_size - 2.)),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Position::new(col, row));
                },

                Some(Tile::Adjacent(adjacent_value)) => {
                    commands.spawn_bundle(Text2dBundle {
                        transform: Transform::from_xyz(x, y, 0.),
                        text: Text {
                            sections: vec![TextSection {
                                value: adjacent_value.to_string(),
                                style: TextStyle {
                                    color: Color::rgb(1., 1., 1.),
                                    font: font.clone(),
                                    font_size: board.sprite_size,
                                },
                            }],
                            alignment: TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                        },
                        ..default()
                    })
                    .insert(Position::new(col, row));
                },

                _ => panic!("Somehow iterated a tile that doesn't exist"),
            }
        }
    }

    commands.insert_resource(board);
    commands.insert_resource(GameState::new());
}

fn has_won(
    board: &ResMut<Board>,
) -> bool {
    for (pos, tile) in board.iter() {
        let covered = board.get_covered_by_pos(&pos);

        if covered.is_some() && *tile != Tile::Bomb {
            return false;
        }
    }

    true
}

fn mouse_click_system(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut commands: Commands,
    board: ResMut<Board>,
    game_state: Res<GameState>,
    asset_server: Res<AssetServer>,
) -> Option<()> {
    if game_state.complete { return Some(()); }

    let position = windows.get_primary()?.cursor_position()?;

    let x = (position.x / board.sprite_size).floor() as usize;
    let y = (board.height as f32 - position.y / board.sprite_size).floor() as usize;


    if mouse_button_input.just_released(MouseButton::Left) {
        if let Some(_) = board.get_flagged_by_pos(&Position::new(x, y)) {
            return Some(());
        }

        board.get_covered_by_pos(&Position::new(x, y)).map(|entity| {
            commands.entity(*entity).insert(ToUncover);
        });
    } else if mouse_button_input.just_released(MouseButton::Right) {
        if let Some(_) = board.get_flagged_by_pos(&Position::new(x, y)) {
            delete_flag(commands, board, x, y);
            return Some(());
        }
        add_flag(commands, board, asset_server, x, y);
    }

    Some(())
}

fn delete_flag(
    mut commands: Commands,
    mut board: ResMut<Board>,
    x: usize,
    y: usize,
) {
    if let Some(flag) = board.get_flagged(y, x) {
        commands.entity(*flag).despawn();
        board.remove_flagged(y, x);
    }
}


fn add_flag(
    mut commands: Commands,
    mut board: ResMut<Board>,
    asset_server: Res<AssetServer>,
    x: usize,
    y: usize,
) {
    let pos_x = ((board.width as f32 / -2.) + x as f32) * board.sprite_size + board.sprite_size / 2.;
    let pos_y = ((board.height as f32 / 2.) - y as f32) * board.sprite_size - board.sprite_size / 2.;

    let entity = commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("flag.png"),
        transform: Transform::from_xyz(pos_x, pos_y, 2.),
        sprite: Sprite {
            custom_size: Some(Vec2::new(board.sprite_size - 2., board.sprite_size - 2.)),
            ..default()
        },
        ..default()
    }).id();
    board.add_flagged(y, x, entity);
}

fn uncover_system(
    mut commands: Commands,
    query: Query<(Entity, &Position), With<ToUncover>>,
    mut board: ResMut<Board>,
    mut game_state: ResMut<GameState>,
    asset_server: Res<AssetServer>,
) {
    for (e, pos) in query.iter() {
        let tile = board.get(pos.y, pos.x).unwrap();

        commands.entity(e).despawn_recursive();
        board.remove_covered_by_pos(pos);

        if has_won(&board) {
            game_state.complete = true;

            display_fullscreen_text(
                commands,
                asset_server,
                "YOU\nWON".to_string(),
                Color::rgb(0., 1., 0.),
                board.sprite_size * 5.
            );
            return;
        }

        match tile {
            Tile::Adjacent(0) => {
                for p in pos.get_surrounding(board.width, board.height) {
                    board.get_covered_by_pos(&p).map(|entity| {
                        commands.entity(*entity).insert(ToUncover);
                    });
                }
            },
            Tile::Bomb => {
                game_state.complete = true;

                for (pos, tile) in board.iter() {
                    if *tile == Tile::Bomb {
                        let covered = board.get_covered_by_pos(&pos);
                        let flagged = board.get_flagged_by_pos(&pos);

                        match (covered, flagged) {
                            (Some(entity), None) => commands.entity(*entity).despawn(),
                            _ => {}
                        }
                    }
                }

                display_fullscreen_text(
                    commands,
                    asset_server,
                    "GAME\nOVER".to_string(),
                    Color::rgb(1., 0., 0.),
                    board.sprite_size * 5.
                );
                return;
            },
            _ => {},
        }
    }
}

fn display_fullscreen_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    message: String,
    color: Color,
    font_size: f32,
) {
    let font = asset_server.load("FiraMono-Regular.ttf");
    commands.spawn_bundle(Text2dBundle {
        transform: Transform::from_xyz(0., 0., 100.),
        text: Text {
            sections: vec![TextSection {
                value: message,
                style: TextStyle {
                    color: color,
                    font: font.clone(),
                    font_size: font_size,
                },
            }],
            alignment: TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        },
        ..default()
    });
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "minesweeper.rs".to_string(),
            width: 1000.,
            height: 1000.,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(mouse_click_system.chain(|_| {})) /* Nom errors like clicking outside the screen */
        .add_system(uncover_system)
        .run();
}
