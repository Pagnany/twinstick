use bevy::prelude::*;
use std::f32::consts::PI;
use std::time::Duration;

pub const SHIELD_ORBIT_RADIUS: f32 = 55.0;
pub const SHIELD_ORB_COUNT: usize = 4;

#[derive(Component)]
pub struct Player {
    pub health: i32,
    pub shield: i32,
    pub radius: f32,
    pub velocity: Vec2,
    /// Angle in radians
    pub angle: f32,
    pub move_acceleration: f32,
    pub friction: f32,
    pub shoot_cooldown_std: Timer,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            health: 1000,
            shield: 0,
            radius: 15.0,
            velocity: Vec2::ZERO,
            angle: 0.0,
            move_acceleration: 15000.0,
            friction: 15.0,
            shoot_cooldown_std: Timer::new(Duration::from_millis(200), TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub struct ShieldOrb {
    pub angle: f32,
    pub radius: f32,
}

pub fn player_movement_system(time: Res<Time>, mut query: Query<(&mut Player, &mut Transform)>) {
    let (mut player, mut transform) = query.single_mut().unwrap();

    player.shoot_cooldown_std.tick(time.delta());

    // Update position
    // Clamp to window bounds
    transform.translation.x += player.velocity.x * time.delta_secs();
    if transform.translation.x > crate::WINDOW_WIDTH / 2.0 {
        transform.translation.x = crate::WINDOW_WIDTH / 2.0;
    } else if transform.translation.x < -crate::WINDOW_WIDTH / 2.0 {
        transform.translation.x = -crate::WINDOW_WIDTH / 2.0;
    }
    transform.translation.y += player.velocity.y * time.delta_secs();
    if transform.translation.y > crate::WINDOW_HEIGHT / 2.0 {
        transform.translation.y = crate::WINDOW_HEIGHT / 2.0;
    } else if transform.translation.y < -crate::WINDOW_HEIGHT / 2.0 {
        transform.translation.y = -crate::WINDOW_HEIGHT / 2.0;
    }

    // texture is rotated
    // so PI / 2.0 is subtracted
    transform.rotation = Quat::from_rotation_z(player.angle - PI / 2.0);

    player.velocity.x -= player.velocity.x * player.friction * time.delta_secs();
    player.velocity.y -= player.velocity.y * player.friction * time.delta_secs();
}

pub fn shield_orbital_system(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut orb_query: Query<(&mut Transform, &mut ShieldOrb), Without<Player>>,
) {
    let player_transform = player_query.single().unwrap();
    let player_pos = player_transform.translation.truncate();

    for (mut transform, mut orb) in orb_query.iter_mut() {
        orb.angle = (orb.angle + time.delta_secs() * 2.0).rem_euclid(std::f32::consts::TAU);
        let offset = Vec2::from_angle(orb.angle) * SHIELD_ORBIT_RADIUS;
        transform.translation.x = player_pos.x + offset.x;
        transform.translation.y = player_pos.y + offset.y;
    }
}
