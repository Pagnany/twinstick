use bevy::prelude::*;
use rand::RngExt;

use crate::player::Player;
use crate::projectile::Projectile;

#[derive(Component)]
pub struct Enemy {
    pub radius: f32,
    pub health: i32,
    pub speed: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            radius: 20.0,
            health: 100,
            speed: 150.0,
        }
    }
}

pub fn enemy_player_collision_system(
    player_query: Query<(&Transform, &Player), With<Player>>,
    enemy_query: Query<(&Transform, &Enemy), Without<Player>>,
    mut game_over: ResMut<crate::GameOver>,
) {
    let (player_transform, player) = player_query.single().unwrap();
    let player_pos = player_transform.translation.truncate();

    for (enemy_transform, enemy) in enemy_query.iter() {
        let enemy_pos = enemy_transform.translation.truncate();
        let distance = player_pos.distance(enemy_pos);

        if distance <= player.radius + enemy.radius {
            game_over.is_game_over = true;
        }
    }
}

pub fn enemy_movement_system(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Transform, &Enemy), Without<Player>>,
) {
    let player_transform = player_query.single().unwrap();
    let player_pos = player_transform.translation.truncate();

    for (mut transform, enemy) in enemy_query.iter_mut() {
        let enemy_pos = transform.translation.truncate();
        let direction = (player_pos - enemy_pos).normalize();

        transform.translation.x += direction.x * enemy.speed * time.delta_secs();
        transform.translation.y += direction.y * enemy.speed * time.delta_secs();
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

pub fn enemy_spawn_system(mut commands: Commands, enemy_query: Query<(&Transform, &Enemy)>) {
    let max_enemies = 5;
    let enemy_count = enemy_query.iter().count();
    if enemy_count >= max_enemies {
        return;
    }
    let mut rng = rand::rngs::ThreadRng::default();
    let side = rng.random_range(0..=3);
    let (x, y) = match side {
        0 => (
            -crate::WINDOW_WIDTH / 2.0 - 100.0,
            rng.random_range(
                -crate::WINDOW_HEIGHT / 2.0 - 100.0..=crate::WINDOW_HEIGHT / 2.0 + 100.0,
            ),
        ),
        1 => (
            crate::WINDOW_WIDTH / 2.0 + 100.0,
            rng.random_range(
                -crate::WINDOW_HEIGHT / 2.0 - 100.0..=crate::WINDOW_HEIGHT / 2.0 + 100.0,
            ),
        ),
        2 => (
            rng.random_range(
                -crate::WINDOW_WIDTH / 2.0 - 100.0..=crate::WINDOW_WIDTH / 2.0 + 100.0,
            ),
            -crate::WINDOW_HEIGHT / 2.0 - 100.0,
        ),
        _ => (
            rng.random_range(
                -crate::WINDOW_WIDTH / 2.0 - 100.0..=crate::WINDOW_WIDTH / 2.0 + 100.0,
            ),
            crate::WINDOW_HEIGHT / 2.0 + 100.0,
        ),
    };

    commands.spawn((
        Sprite::from_color(Color::srgb(0.8, 0.2, 0.2), Vec2::new(40.0, 40.0)),
        Transform::from_xyz(x, y, -0.1),
        Enemy::default(),
    ));
}
