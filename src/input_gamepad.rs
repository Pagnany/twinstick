use crate::player::Player;
use bevy::{input::gamepad::GamepadEvent, prelude::*};

pub fn gamepad_system(
    time: Res<Time>,
    gamepads: Query<&Gamepad>,
    mut player_query: Query<&mut Player>,
) {
    let mut player = player_query.single_mut().unwrap();

    for gamepad in &gamepads {
        let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
        let left_stick_y = gamepad.get(GamepadAxis::LeftStickY).unwrap();
        if left_stick_x.abs() > 0.1 || left_stick_y.abs() > 0.1 {
            player.velocity.x += left_stick_x * player.move_acceleration * time.delta_secs();
            player.velocity.y += left_stick_y * player.move_acceleration * time.delta_secs();
        }

        let right_stick_x = gamepad.get(GamepadAxis::RightStickX).unwrap();
        let right_stick_y = gamepad.get(GamepadAxis::RightStickY).unwrap();
        if right_stick_x.abs() > 0.1 || right_stick_y.abs() > 0.1 {
            player.angle = right_stick_y.atan2(right_stick_x);
        }
    }
}

/// Show all gamepad input events in the log
pub fn gamepad_input_all_events(mut evr_gamepad: MessageReader<GamepadEvent>) {
    for ev in evr_gamepad.read() {
        info!("Gamepad event: {:?}", ev);
    }
}
