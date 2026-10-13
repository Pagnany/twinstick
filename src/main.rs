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

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Component)]
struct ScoreDisplay;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

#[derive(Component)]
struct MenuScreen;

#[derive(Component)]
struct GameOverScreen;

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct ReturnToMenuButton;

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
    app.init_resource::<Score>();
    app.init_state::<GameState>();
    app.add_systems(Startup, setup_camera);
    app.add_systems(
        Update,
        (systems::kill_game_on_esc, start_button_system, return_to_menu_button_system),
    );
    app.add_systems(
        FixedUpdate,
        (
            enemy::shield_orb_enemy_collision_system,
            enemy::projectile_enemy_collision_system,
            enemy::enemy_player_collision_system,
            enemy::enemy_spawn_system,
        )
            .chain()
            .run_if(in_state(GameState::Playing)),
    );
    app.add_systems(
        Update,
        (
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
        )
            .run_if(in_state(GameState::Playing)),
    );
    app.add_systems(OnEnter(GameState::Menu), spawn_menu_screen);
    app.add_systems(OnExit(GameState::Menu), despawn_menu_screen);
    app.add_systems(OnEnter(GameState::Playing), spawn_gameplay);
    app.add_systems(OnExit(GameState::Playing), despawn_gameplay);
    app.add_systems(OnEnter(GameState::GameOver), spawn_game_over_screen);
    app.add_systems(OnExit(GameState::GameOver), despawn_game_over_screen);
    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_menu_screen(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(32.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.025, 0.035, 0.07)),
            MenuScreen,
        ))
        .with_children(|screen| {
            screen.spawn((
                Text::new("TWINSTICK"),
                TextFont {
                    font_size: FontSize::Px(80.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            screen
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(280.0),
                        height: Val::Px(90.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.12, 0.42, 0.72)),
                    StartButton,
                ))
                .with_child((
                    Text::new("START"),
                    TextFont {
                        font_size: FontSize::Px(40.0),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
        });
}

fn despawn_menu_screen(mut commands: Commands, screen: Query<Entity, With<MenuScreen>>) {
    for entity in &screen {
        commands.entity(entity).despawn();
    }
}

fn spawn_gameplay(mut commands: Commands, asset_server: Res<AssetServer>, mut score: ResMut<Score>) {
    score.value = 0;
    let player_texture = asset_server.load("textures/player01.png");

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

fn despawn_gameplay(
    mut commands: Commands,
    player_entities: Query<Entity, With<player::Player>>,
    orb_entities: Query<Entity, With<player::ShieldOrb>>,
    enemy_entities: Query<Entity, With<enemy::Enemy>>,
    projectile_entities: Query<Entity, With<projectile::Projectile>>,
    score_displays: Query<Entity, With<ScoreDisplay>>,
) {
    for entity in player_entities
        .iter()
        .chain(orb_entities.iter())
        .chain(enemy_entities.iter())
        .chain(projectile_entities.iter())
        .chain(score_displays.iter())
    {
        commands.entity(entity).despawn();
    }
}

fn spawn_game_over_screen(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(32.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.025, 0.035, 0.07, 0.96)),
            GameOverScreen,
        ))
        .with_children(|screen| {
            screen.spawn((
                Text::new("GAME OVER"),
                TextFont {
                    font_size: FontSize::Px(80.0),
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.3, 0.3)),
            ));
            screen
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(280.0),
                        height: Val::Px(90.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.12, 0.42, 0.72)),
                    ReturnToMenuButton,
                ))
                .with_child((
                    Text::new("MAIN MENU"),
                    TextFont {
                        font_size: FontSize::Px(32.0),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
        });
}

fn despawn_game_over_screen(mut commands: Commands, screen: Query<Entity, With<GameOverScreen>>) {
    for entity in &screen {
        commands.entity(entity).despawn();
    }
}

fn start_button_system(
    buttons: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        next_state.set(GameState::Playing);
    }
}

fn return_to_menu_button_system(
    buttons: Query<&Interaction, (Changed<Interaction>, With<ReturnToMenuButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        next_state.set(GameState::Menu);
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
