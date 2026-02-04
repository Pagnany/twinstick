use bevy::prelude::*;

use crate::projectile::Projectile;

#[derive(Component)]
pub struct Enemy {
    pub radius: f32,
    pub health: i32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            radius: 20.0,
            health: 100,
        }
    }
}

pub fn projectile_enemy_collision_system(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform, &Projectile)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy)>,
) {
    for (projectile_entity, projectile_transform, projectile) in projectile_query.iter() {
        if !projectile.from_player {
            continue;
        }

        for (enemy_entity, enemy_transform, mut enemy) in enemy_query.iter_mut() {
            let distance = projectile_transform
                .translation
                .truncate()
                .distance(enemy_transform.translation.truncate());
            if distance <= enemy.radius + projectile.radius {
                enemy.health -= projectile.damage;
                if enemy.health <= 0 {
                    commands.entity(enemy_entity).despawn();
                }
                commands.entity(projectile_entity).despawn();
                break;
            }
        }
    }
}
