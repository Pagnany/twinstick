use bevy::prelude::*;

pub fn kill_game_on_esc(keys: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if keys.pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

pub fn game_over_system(game_over: Res<crate::GameOver>, mut exit: MessageWriter<AppExit>) {
    if game_over.is_game_over {
        exit.write(AppExit::Success);
    }
}
