use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowResolution, WindowTheme},
};
use std::time::Duration;

pub mod enemy;
pub mod input_gamepad;
pub mod input_keyboard_mouse;
pub mod menu;
pub mod obstacle;
pub mod player;
pub mod projectile;
pub mod systems;

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource, Default)]
struct SurvivalTime {
    elapsed: Duration,
    displayed_seconds: u64,
}

#[derive(Resource, Default, Clone, Copy, PartialEq, Eq)]
enum InputMethod {
    #[default]
    KeyboardMouse,
    Gamepad,
}

#[derive(Component)]
struct ScoreDisplay;

#[derive(Component)]
struct SurvivalTimeDisplay;

#[derive(Component)]
struct SurvivalTimeText;

#[derive(Component)]
struct InputMethodCheckbox;

#[derive(Component)]
struct InputMethodCheckboxMark;

#[derive(Component)]
struct InputMethodCheckboxBox;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

const WINDOW_TITLE: &str = "Twinstick";
pub const WINDOW_WIDTH: f32 = 1920.0;
pub const WINDOW_HEIGHT: f32 = 1080.0;

const UPDATE_INTERVAL: f64 = 1.0 / 50.0;

fn main() {
    // #[cfg(target_os = "windows")]
    // std::env::set_var("WGPU_BACKEND", "dx12");

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
    app.init_resource::<SurvivalTime>();
    app.init_resource::<enemy::PositionEnemySpawnTimer>();
    app.init_resource::<InputMethod>();
    app.init_state::<GameState>();
    app.add_systems(Startup, setup_camera);
    app.add_systems(
        Update,
        (
            systems::kill_game_on_esc,
            menu::start_button_system,
            menu::return_to_menu_button_system,
            toggle_input_method,
            update_input_method_checkbox,
        ),
    );
    app.add_systems(
        FixedUpdate,
        (
            enemy::shield_orb_enemy_collision_system,
            enemy::projectile_enemy_collision_system,
            enemy::enemy_player_collision_system,
            obstacle::obstacle_player_collision_system,
            enemy::enemy_spawn_system,
            enemy::position_enemy_spawn_system,
            obstacle::obstacle_spawn_system,
        )
            .chain()
            .run_if(in_state(GameState::Playing)),
    );
    app.add_systems(
        Update,
        (
            input_gamepad::gamepad_movement_system,
            input_gamepad::gamepad_aim_system,
            input_gamepad::gamepad_shoot_system,
            // input_gamepad::gamepad_input_all_events,
        )
            .run_if(in_state(GameState::Playing))
            .run_if(use_gamepad),
    );
    app.add_systems(
        Update,
        (
            player::player_movement_system,
            player::shield_orbital_system,
            projectile::projectile_movement_system,
            enemy::enemy_movement_system,
            obstacle::obstacle_movement_system,
            update_score_display,
            update_survival_time_display,
        )
            .run_if(in_state(GameState::Playing)),
    );
    app.add_systems(
        Update,
        (
            input_keyboard_mouse::keyboard_move_system,
            input_keyboard_mouse::mouse_aim_system,
            input_keyboard_mouse::mouse_shoot_projectile,
        )
            .run_if(in_state(GameState::Playing))
            .run_if(use_keyboard_mouse),
    );
    app.add_systems(
        OnEnter(GameState::Menu),
        (menu::spawn_menu_screen, spawn_input_method_checkbox),
    );
    app.add_systems(
        OnExit(GameState::Menu),
        (menu::despawn_menu_screen, despawn_input_method_checkbox),
    );
    app.add_systems(
        OnEnter(GameState::Playing),
        (
            spawn_gameplay,
            enemy::reset_position_enemy_spawn_timer,
            spawn_input_method_checkbox,
        ),
    );
    app.add_systems(
        OnExit(GameState::Playing),
        (despawn_gameplay, despawn_input_method_checkbox),
    );
    app.add_systems(OnEnter(GameState::GameOver), menu::spawn_game_over_screen);
    app.add_systems(OnExit(GameState::GameOver), menu::despawn_game_over_screen);
    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn use_gamepad(input_method: Res<InputMethod>) -> bool {
    *input_method == InputMethod::Gamepad
}

fn use_keyboard_mouse(input_method: Res<InputMethod>) -> bool {
    *input_method == InputMethod::KeyboardMouse
}

fn spawn_input_method_checkbox(mut commands: Commands, input_method: Res<InputMethod>) {
    commands
        .spawn((
            Button,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                right: Val::Px(20.0),
                width: Val::Px(190.0),
                height: Val::Px(48.0),
                padding: UiRect::all(Val::Px(10.0)),
                column_gap: Val::Px(10.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..default()
            },
            GlobalZIndex(1),
            BackgroundColor(Color::srgb(0.12, 0.16, 0.24)),
            InputMethodCheckbox,
        ))
        .with_children(|checkbox| {
            checkbox
                .spawn((
                    Node {
                        width: Val::Px(24.0),
                        height: Val::Px(24.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    InputMethodCheckboxBox,
                    BackgroundColor(if *input_method == InputMethod::Gamepad {
                        Color::srgb(0.12, 0.65, 0.32)
                    } else {
                        Color::srgb(0.28, 0.32, 0.4)
                    }),
                ))
                .with_child((
                    Text::new(if *input_method == InputMethod::Gamepad {
                        "✓"
                    } else {
                        ""
                    }),
                    TextFont {
                        font_size: FontSize::Px(18.0),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    InputMethodCheckboxMark,
                ));
            checkbox.spawn((
                Text::new("Use gamepad"),
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn despawn_input_method_checkbox(
    mut commands: Commands,
    checkbox: Query<Entity, With<InputMethodCheckbox>>,
) {
    for entity in &checkbox {
        commands.entity(entity).despawn();
    }
}

fn toggle_input_method(
    checkboxes: Query<&Interaction, (Changed<Interaction>, With<InputMethodCheckbox>)>,
    mut input_method: ResMut<InputMethod>,
) {
    if checkboxes
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        *input_method = match *input_method {
            InputMethod::KeyboardMouse => InputMethod::Gamepad,
            InputMethod::Gamepad => InputMethod::KeyboardMouse,
        };
    }
}

fn update_input_method_checkbox(
    input_method: Res<InputMethod>,
    mut marks: Query<&mut Text, With<InputMethodCheckboxMark>>,
    mut boxes: Query<&mut BackgroundColor, With<InputMethodCheckboxBox>>,
) {
    if !input_method.is_changed() {
        return;
    }

    let checked = *input_method == InputMethod::Gamepad;
    for mut mark in &mut marks {
        **mark = if checked {
            "✓".to_owned()
        } else {
            String::new()
        };
    }
    for mut background in &mut boxes {
        **background = if checked {
            Color::srgb(0.12, 0.65, 0.32).into()
        } else {
            Color::srgb(0.28, 0.32, 0.4).into()
        };
    }
}

fn spawn_gameplay(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
    mut survival_time: ResMut<SurvivalTime>,
) {
    score.value = 0;
    *survival_time = SurvivalTime::default();
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

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            SurvivalTimeDisplay,
        ))
        .with_child((
            Text::new("Time: 0s"),
            TextFont {
                font_size: FontSize::Px(32.0),
                ..default()
            },
            TextColor(Color::WHITE),
            SurvivalTimeText,
        ));

    commands.spawn((
        Sprite::from_image(player_texture),
        player::Player::default(),
    ));

    for index in 0..player::SHIELD_ORB_COUNT {
        let angle = index as f32 * std::f32::consts::TAU / player::SHIELD_ORB_COUNT as f32;
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
    obstacle_entities: Query<Entity, With<obstacle::Obstacle>>,
    projectile_entities: Query<Entity, With<projectile::Projectile>>,
    score_displays: Query<Entity, With<ScoreDisplay>>,
    survival_time_displays: Query<Entity, With<SurvivalTimeDisplay>>,
) {
    for entity in player_entities
        .iter()
        .chain(orb_entities.iter())
        .chain(enemy_entities.iter())
        .chain(obstacle_entities.iter())
        .chain(projectile_entities.iter())
        .chain(score_displays.iter())
        .chain(survival_time_displays.iter())
    {
        commands.entity(entity).despawn();
    }
}

fn update_survival_time_display(
    time: Res<Time>,
    mut survival_time: ResMut<SurvivalTime>,
    mut display: Query<&mut Text, With<SurvivalTimeText>>,
) {
    survival_time.elapsed += time.delta();
    let elapsed_seconds = survival_time.elapsed.as_secs();
    if elapsed_seconds == survival_time.displayed_seconds {
        return;
    }

    survival_time.displayed_seconds = elapsed_seconds;
    for mut text in &mut display {
        **text = format!("Time: {elapsed_seconds}s");
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
