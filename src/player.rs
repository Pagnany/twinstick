use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub radius: f32,
    pub velocity: Vec2,
    /// Angle in radians
    pub angle: f32,
    pub move_acceleration: f32,
    pub dash_power: f32,
    pub friction: f32,
    pub breaking_power: f32,
    pub right_trigger_down: bool,
    pub init_left_trigger_down: bool,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            radius: 15.0,
            velocity: Vec2::ZERO,
            angle: 0.0,
            move_acceleration: 15000.0,
            dash_power: 5000.0,
            friction: 15.0,
            breaking_power: 10000.0,
            right_trigger_down: false,
            init_left_trigger_down: false,
        }
    }
}

pub fn player_movement_system(time: Res<Time>, mut query: Query<(&mut Player, &mut Transform)>) {
    let (mut player, mut transform) = query.single_mut().unwrap();

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

    // Rotate the player based on velocity
    if player.velocity.length() > 0.0 {
        player.angle = player.velocity.y.atan2(player.velocity.x);

        // texture is rotated
        // so PI / 2.0 is subtracted
        transform.rotation = Quat::from_rotation_z(player.angle - crate::PI / 2.0);
    }

    player.velocity.x -= player.velocity.x * player.friction * time.delta_secs();
    player.velocity.y -= player.velocity.y * player.friction * time.delta_secs();
}
