use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowResolution, WindowTheme},
};

pub mod input_gamepad;
pub mod player;
pub mod systems;

pub const PI: f32 = std::f32::consts::PI;

const WINDOW_TITLE: &str = "Twinstick";
pub const WINDOW_WIDTH: f32 = 1920.0;
pub const WINDOW_HEIGHT: f32 = 1080.0;

const UPDATE_INTERVAL: f64 = 1.0 / 50.0;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: WINDOW_TITLE.into(),
                name: Some(WINDOW_TITLE.into()),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                resizable: false,
                resolution: WindowResolution::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
                enabled_buttons: EnabledButtons {
                    close: true,
                    maximize: false,
                    minimize: false,
                },
                window_theme: Some(WindowTheme::Dark),
                ..default()
            }),
            ..default()
        })
        .set(AssetPlugin {
            meta_check: bevy::asset::AssetMetaCheck::Never,
            ..default()
        }),));
    app.insert_resource(Time::<Fixed>::from_seconds(UPDATE_INTERVAL));
    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        (
            systems::kill_game_on_esc,
            input_gamepad::gamepad_system,
            player::player_movement_system,
        ),
    );
    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_texture = asset_server.load("textures/player01.png");

    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_image(player_texture),
        player::Player::default(),
    ));
}
