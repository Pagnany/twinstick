use bevy::prelude::*;

pub fn kill_game_on_esc(keys: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if keys.pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
