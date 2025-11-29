use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec2,
    pub damage: f32,
    pub from_player: bool,
}

pub fn projectile_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &Projectile)>,
) {
    for (entity, mut transform, projectile) in query.iter_mut() {
        transform.translation.x += projectile.velocity.x * time.delta_secs();
        transform.translation.y += projectile.velocity.y * time.delta_secs();

        // if outside of window, despawn
        if transform.translation.x > crate::WINDOW_WIDTH / 2.0
            || transform.translation.x < -crate::WINDOW_WIDTH / 2.0
            || transform.translation.y > crate::WINDOW_HEIGHT / 2.0
            || transform.translation.y < -crate::WINDOW_HEIGHT / 2.0
        {
            commands.entity(entity).despawn();
        }
    }
}
