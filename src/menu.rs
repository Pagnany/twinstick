use bevy::prelude::*;

use crate::GameState;

#[derive(Component)]
pub struct MenuScreen;

#[derive(Component)]
pub struct GameOverScreen;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct ReturnToMenuButton;

pub fn spawn_menu_screen(mut commands: Commands) {
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

pub fn despawn_menu_screen(mut commands: Commands, screen: Query<Entity, With<MenuScreen>>) {
    for entity in &screen {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_game_over_screen(mut commands: Commands) {
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

pub fn despawn_game_over_screen(
    mut commands: Commands,
    screen: Query<Entity, With<GameOverScreen>>,
) {
    for entity in &screen {
        commands.entity(entity).despawn();
    }
}

pub fn start_button_system(
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

pub fn return_to_menu_button_system(
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
