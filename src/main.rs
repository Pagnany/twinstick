use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowResolution, WindowTheme},
};

pub mod systems;

const WINDOW_TITLE: &str = "Twinstick";
pub const WINDOW_WIDTH: u32 = 1920;
pub const WINDOW_HEIGHT: u32 = 1080;

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
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
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
    app.add_systems(Update, systems::kill_game_on_esc);
    app.run();
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
}
