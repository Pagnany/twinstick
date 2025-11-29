use crate::player::Player;
use crate::projectile::Projectile;
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

pub fn mouse_aim_system(
    window: Single<&Window, With<PrimaryWindow>>,
    mut player_query: Query<(&mut Player, &Transform)>,
) {
    let (mut player, player_trans) = player_query.single_mut().unwrap();
    // Rotate player to face mouse cursor
    // player coordinates are centered at (0,0)
    // window coordinates are top-left at (0,0)
    if let Some(mouse_pos) = window.cursor_position() {
        let player_pos = Vec2::new(
            player_trans.translation.x + crate::WINDOW_WIDTH / 2.0,
            (player_trans.translation.y - crate::WINDOW_HEIGHT / 2.0).abs(),
        );
        let direction = mouse_pos - player_pos;
        player.angle = -direction.y.atan2(direction.x);
    }
}

pub fn mouse_shoot_projectile(
    mut commands: Commands,
    time: Res<Time>,
    mut player: Query<(&Transform, &mut Player)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    let (transform, mut player) = player.single_mut().unwrap();

    player.shoot_cooldown.tick(time.delta());

    if mouse_input.pressed(MouseButton::Left) && player.shoot_cooldown.is_finished() {
        player.shoot_cooldown.reset();

        let projectile_speed = 1500.0;
        let velocity = Vec2::new(
            projectile_speed * player.angle.cos(),
            projectile_speed * player.angle.sin(),
        );

        commands.spawn((
            Sprite::from_color(Color::srgb(0.8, 0.2, 0.2), Vec2::new(10.0, 10.0)),
            Transform::from_xyz(transform.translation.x, transform.translation.y, 0.0),
            Projectile {
                velocity,
                damage: 10.0,
                from_player: true,
            },
        ));
    }
}
