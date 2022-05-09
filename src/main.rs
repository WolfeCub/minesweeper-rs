use bevy::{prelude::*, window::PresentMode, sprite::SPRITE_SHADER_HANDLE, utils::HashMap};
use rand::Rng;

#[derive(Component, Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

impl Position {
    fn get_surrounding(&self, width: usize, height: usize) -> Vec<Position> {
        let irow = self.y as i32;
        let icol = self.x as i32;

        /* TODO: Probably a faster way of doing this. Generators? */
        let mut result: Vec<Position> = Vec::new();

        for r in irow-1..=irow+1 {
            for c in icol-1..=icol+1 {
                if r < 0 || r >= height as i32 || c < 0 || c >= width as i32 {
                    continue;
                }

                result.push(Position::new(c as usize, r as usize));
            }
        }
        result
    }

    fn get_direct_adjacent(&self, width: usize, height: usize) -> Vec<Position> {
        let irow = self.y as i32;
        let icol = self.x as i32;

        /* TODO: Probably a faster way of doing this. Generators? */
        let mut result: Vec<Position> = Vec::new();

        for (r, c) in vec![(irow-1, icol), (irow+1, icol), (irow, icol-1), (irow, icol+1)] {
                if r < 0 || r >= height as i32 || c < 0 || c >= width as i32 {
                    continue;
            }

            result.push(Position::new(c as usize, r as usize));
        }
        result
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Adjacent(u8),
    Bomb,
}

impl ToString for Tile {
    fn to_string(&self) -> String {
        match self {
            Tile::Bomb => "B".to_string(),
            Tile::Adjacent(val) => val.to_string(),
        }
    }
}

struct Board {
    pub height: usize,
    pub width: usize,
    pub sprite_size: f32,
    pub grid: Vec<Vec<Tile>>,
    pub covered: HashMap<Position, Entity>,
    pub flags: HashMap<Position, Entity>,
}

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

impl Board {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            sprite_size: 50., /* TODO: Configurable? */
            grid: vec![vec![Tile::Adjacent(0); width]; height],
            covered: HashMap::new(),
            flags: HashMap::new(),
        }
    }

    fn get(&self, row: usize, col: usize) -> Option<Tile> {
        Some(self.grid.get(row)?.get(col)?.clone())
    }

    fn add_bomb(&mut self) {
        let mut rng = rand::thread_rng();
        let mut row;
        let mut col;

        loop {
            row = rng.gen_range(0..self.height);
            col = rng.gen_range(0..self.width);

            if let Tile::Adjacent(_) = self.grid[row][col] {
                self.grid[row][col] = Tile::Bomb;
                break;
            }
        }

        for p in Position::new(col, row).get_surrounding(self.width, self.height) {
            self.get(p.y, p.x).map(|t| {
                self.grid[p.y][p.x] = match t {
                    Tile::Adjacent(val) => Tile::Adjacent(val + 1),
                    other => other
                };
            });
        }
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        self.grid.iter().map(|row| {
            row.iter().map(|item| item.to_string()).collect::<Vec<String>>().join(",")
        }).collect::<Vec<String>>().join("\n")
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                    custom_size: Some(Vec2::new(board.sprite_size, board.sprite_size)),
                    ..default()
                },
                ..default()
            })
            .insert(Position::new(col, row))
            .id();

            board.covered.insert(Position::new(col, row), covered_entity);

            match board.get(row, col) {
                Some(Tile::Bomb) => {
                    commands.spawn_bundle(SpriteBundle {
                        texture: asset_server.load("bomb.png"),
                        transform: Transform::from_xyz(x, y, 0.),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(board.sprite_size, board.sprite_size)),
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

fn mouse_click_system(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut commands: Commands,
    board: ResMut<Board>,
    game_state: Res<GameState>,
    asset_server: Res<AssetServer>,
) {
    if game_state.complete { return; }

    /* TODO: Make this nicer. Ideally with `?` syntax */
    let primary_window = windows.get_primary();
    if primary_window.is_none() { return; }
    let position_option = primary_window.unwrap().cursor_position();
    if position_option.is_none() { return; }

    let position = position_option.unwrap();
    
    let x = (position.x / board.sprite_size).floor() as usize;
    let y = (board.height as f32 - position.y / board.sprite_size).floor() as usize;


    if mouse_button_input.just_released(MouseButton::Left) {
        if let Some(_) = board.flags.get(&Position::new(x, y)) {
            return;
        }

        board.covered.get(&Position::new(x, y)).map(|entity| {
            commands.entity(*entity).insert(ToUncover);
        });
    } else if mouse_button_input.just_released(MouseButton::Right) {
        if let Some(_) = board.flags.get(&Position::new(x, y)) {
            delete_flag(commands, board, x, y);
            return;
        }
        add_flag(commands, board, asset_server, x, y);
    }
}

fn delete_flag(
    mut commands: Commands,
    mut board: ResMut<Board>,
    x: usize,
    y: usize,
) {
    let pos = &Position::new(x, y);
    if let Some(flag) = board.flags.get(pos) {
        commands.entity(*flag).despawn();
        board.flags.remove(pos);
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
            custom_size: Some(Vec2::new(board.sprite_size, board.sprite_size)),
            ..default()
        },
        ..default()
    }).id();
    board.flags.insert(Position::new(x, y), entity);
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
        board.covered.remove(pos);

        match tile {
            Tile::Adjacent(adj) => {
                if adj == 0 {
                    for p in pos.get_direct_adjacent(board.width, board.height) {
                        board.covered.get(&p).map(|entity| {
                            commands.entity(*entity).insert(ToUncover);
                        });
                    }
                }
            },
            Tile::Bomb => {
                game_state.complete = true;

                /* TODO: Make this cleaner */
                for row in 0..board.height {
                    for col in 0..board.width {
                        board.get(row, col).map(|t| {
                            if t == Tile::Bomb {
                                board.covered.get(&Position::new(col, row)).map(|entity| {
                                    commands.entity(*entity).insert(ToUncover);
                                });
                            }
                        });
                    }
                }

                /* TODO: Refactor this out of here */
                let font = asset_server.load("FiraMono-Regular.ttf");
                commands.spawn_bundle(Text2dBundle {
                    transform: Transform::from_xyz(0., 0., 100.),
                    text: Text {
                        sections: vec![TextSection {
                            value: "GAME\nOVER".to_string(),
                            style: TextStyle {
                                color: Color::rgb(1., 0., 0.),
                                font: font.clone(),
                                font_size: board.sprite_size * 5.,
                            },
                        }],
                        alignment: TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    },
                    ..default()
                });
            },
        }
    }
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
        .add_system(mouse_click_system)
        .add_system(uncover_system)
        .run();
}
