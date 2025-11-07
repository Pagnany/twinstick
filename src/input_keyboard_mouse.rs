use crate::player::Player;
use bevy::{
    prelude::*,
    window::{PrimaryWindow, Window},
};

pub fn keyboard_system(time: Res<Time>, mut player_query: Query<&mut Player>) {
    let mut player = player_query.single_mut().unwrap();

    // Player movement
    let left_stick_x = 1.0;
    let left_stick_y = 1.0;
    player.velocity.x += left_stick_x * player.move_acceleration * time.delta_secs();
    player.velocity.y += left_stick_y * player.move_acceleration * time.delta_secs();
}

pub fn keyboard_move_system(
    time: Res<Time>,
    mut player_query: Query<&mut Player>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    const KEYBOARD_ACC: f32 = 1.0;
    let mut player = player_query.single_mut().unwrap();

    if keyboard_input.pressed(KeyCode::KeyA) {
        player.velocity.x -= KEYBOARD_ACC * player.move_acceleration * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        player.velocity.x += KEYBOARD_ACC * player.move_acceleration * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        player.velocity.y += KEYBOARD_ACC * player.move_acceleration * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        player.velocity.y -= KEYBOARD_ACC * player.move_acceleration * time.delta_secs();
    }
}

pub fn mouse_aim_system(window: Single<&Window, With<PrimaryWindow>>) {
    if let Some(position) = window.cursor_position() {
        println!("Cursor is inside the primary window, at {:?}", position);
    } else {
        println!("Cursor is not in the game window.");
    }
}
