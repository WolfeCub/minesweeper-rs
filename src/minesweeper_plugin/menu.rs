use bevy::prelude::*;

use super::{GameState, AppState, Difficulty};

pub struct RootEntity(Entity);

pub fn main_menu_setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    let root = commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..default()
    }).with_children(|root| {
        spawn_menu_option(root, Difficulty::Easy, "Easy", &asset_server);
        spawn_menu_option(root, Difficulty::Medium, "Medium", &asset_server);
        spawn_menu_option(root, Difficulty::Hard, "Hard", &asset_server);
    }).id();

    commands.insert_resource(RootEntity(root));
}

pub fn spawn_menu_option(parent: &mut ChildBuilder, diff: Difficulty, text: &str, asset_server: &Res<AssetServer>) {
    parent.spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: Rect::all(Val::Px(8.0)),
            ..Default::default()
        },
        color: UiColor(Color::rgb(0.7, 0.7, 0.7)),
        ..default()
    }).with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            transform: Transform::from_xyz(0., 0., 100.),
            text: Text {
                sections: vec![TextSection {
                    value: text.to_owned(),
                    style: TextStyle {
                        color: Color::rgb(0.0, 0.0, 0.0),
                        font: asset_server.load("FiraMono-Regular.ttf"),
                        font_size: 30.,
                    },
                }],
                ..default()
            },
            ..default()
        });
    })
    .insert(diff);
}

pub fn main_menu_action_system(
    buttons: Query<(&Interaction, &Difficulty), (Changed<Interaction>, With<Button>)>,
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    root: Res<RootEntity>,
    mut mouse_button_input: ResMut<Input<MouseButton>>,
    mut windows: ResMut<Windows>,
) {
    for (interaction, button) in buttons.iter() {
        if *interaction != Interaction::Clicked { continue; }

        let window = windows.get_primary_mut().unwrap();
        /* TODO: Sync this with the game drawing sprite size */
        let sprite_size = 50.;
        let (width, height, bombs) = match button {
            Difficulty::Easy => (9, 9, 10),
            Difficulty::Medium => (16, 16, 40),
            Difficulty::Hard => (30, 16, 99),
        };

        commands.insert_resource(GameState::new(width, height, bombs));
        window.set_resolution(width as f32 * sprite_size, height as f32 * sprite_size);

        window.set_resizable(false);

        let RootEntity(e) = root.as_ref();
        commands.entity(*e).despawn_recursive();

        state.set(AppState::InGame).expect("Failed to transition state to in game");
        
        /* Eat the mouse press or we'll click "through" into the game */
        mouse_button_input.reset(MouseButton::Left);
    }
}
