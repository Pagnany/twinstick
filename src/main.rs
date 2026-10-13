use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowResolution, WindowTheme},
};

pub mod enemy;
pub mod input_gamepad;
pub mod input_keyboard_mouse;
pub mod player;
pub mod projectile;
pub mod systems;

#[derive(Resource)]
pub struct GameOver {
    pub is_game_over: bool,
}

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Component)]
struct ScoreDisplay;

const WINDOW_TITLE: &str = "Twinstick";
pub const WINDOW_WIDTH: f32 = 1920.0;
pub const WINDOW_HEIGHT: f32 = 1080.0;

const UPDATE_INTERVAL: f64 = 1.0 / 50.0;

fn main() {
    #[cfg(target_os = "windows")]
    std::env::set_var("WGPU_BACKEND", "dx12");

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
    app.insert_resource(GameOver {
        is_game_over: false,
    });
    app.init_resource::<Score>();
    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        (
            systems::kill_game_on_esc,
            // input_gamepad::gamepad_movement_system,
            // input_gamepad::gamepad_aim_system,
            // input_gamepad::gamepad_shoot_system,
            // input_gamepad::gamepad_input_all_events,
            input_keyboard_mouse::keyboard_move_system,
            input_keyboard_mouse::mouse_aim_system,
            input_keyboard_mouse::mouse_shoot_projectile,
            player::player_movement_system,
            player::shield_orbital_system,
            projectile::projectile_movement_system,
            enemy::enemy_movement_system,
            update_score_display,
            systems::game_over_system,
        ),
    );
    app.add_systems(
        FixedUpdate,
        (
            enemy::shield_orb_enemy_collision_system,
            enemy::projectile_enemy_collision_system,
            enemy::enemy_player_collision_system,
            enemy::enemy_spawn_system,
        )
            .chain(),
    );
    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_texture = asset_server.load("textures/player01.png");

    commands.spawn(Camera2d);

    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font_size: FontSize::Px(32.0),
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        ScoreDisplay,
    ));

    commands.spawn((
        Sprite::from_image(player_texture),
        player::Player::default(),
    ));

    for angle in [0.0, std::f32::consts::PI] {
        let offset = Vec2::from_angle(angle) * player::SHIELD_ORBIT_RADIUS;
        commands.spawn((
            Sprite::from_color(Color::srgb(0.2, 0.8, 1.0), Vec2::splat(18.0)),
            Transform::from_xyz(offset.x, offset.y, 0.1),
            player::ShieldOrb { angle, radius: 9.0 },
        ));
    }
}

fn update_score_display(score: Res<Score>, mut display: Query<&mut Text, With<ScoreDisplay>>) {
    if !score.is_changed() {
        return;
    }

    for mut text in &mut display {
        **text = format!("Score: {}", score.value);
    }
}
