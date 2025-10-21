use crate::player::Player;
use bevy::{input::gamepad::GamepadEvent, prelude::*};

pub fn gamepad_system(
    time: Res<Time>,
    gamepads: Query<&Gamepad>,
    mut player_query: Query<&mut Player>,
) {
    let mut player = player_query.single_mut().unwrap();

    for gamepad in &gamepads {
        // The Controller I own works differently on Windows and Linux
        // Controller on Windows
        // let right_trigger = gamepad.get(GamepadButton::RightTrigger2).unwrap();
        // Gamesir controller on Linux
        let right_trigger = gamepad.get(GamepadAxis::RightZ).unwrap();
        if !player.right_trigger_down && right_trigger > 0.2 {
            player.right_trigger_down = true;
            player.velocity.x += player.dash_power * player.angle.cos();
            player.velocity.y += player.dash_power * player.angle.sin();
        } else if player.right_trigger_down && right_trigger < 0.2 {
            player.right_trigger_down = false;
        }

        let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
        let left_stick_y = gamepad.get(GamepadAxis::LeftStickY).unwrap();
        if left_stick_x.abs() > 0.1 || left_stick_y.abs() > 0.1 {
            player.velocity.x += left_stick_x * player.move_acceleration * time.delta_secs();
            player.velocity.y += left_stick_y * player.move_acceleration * time.delta_secs();
        }
    }
}

/// Show all gamepad input events in the log
pub fn gamepad_input_all_events(mut evr_gamepad: MessageReader<GamepadEvent>) {
    for ev in evr_gamepad.read() {
        info!("Gamepad event: {:?}", ev);
    }
}
