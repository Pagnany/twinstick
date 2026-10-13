use bevy::prelude::*;
use rand::RngExt;

use crate::player::{Player, ShieldOrb};
use crate::projectile::Projectile;

#[derive(Component)]
pub struct Enemy {
    pub radius: f32,
    pub health: i32,
    pub speed: f32,
    pub spawn_warning: Option<Timer>,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            radius: 20.0,
            health: 100,
            speed: 150.0,
            spawn_warning: None,
        }
    }
}

impl Enemy {
    fn can_be_damaged(&self) -> bool {
        self.spawn_warning.as_ref().is_none_or(Timer::is_finished)
    }
}

#[derive(Component)]
pub struct PositionSpawnedEnemy;

#[derive(Resource)]
pub struct PositionEnemySpawnTimer(Timer);

impl Default for PositionEnemySpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(5.0, TimerMode::Once))
    }
}

pub fn reset_position_enemy_spawn_timer(mut timer: ResMut<PositionEnemySpawnTimer>) {
    timer.0.reset();
}

pub fn enemy_player_collision_system(
    player_query: Query<(&Transform, &Player), With<Player>>,
    enemy_query: Query<(&Transform, &Enemy), Without<Player>>,
    mut next_state: ResMut<NextState<crate::GameState>>,
) {
    let (player_transform, player) = player_query.single().unwrap();
    let player_pos = player_transform.translation.truncate();

    for (enemy_transform, enemy) in enemy_query.iter() {
        if !enemy.can_be_damaged() {
            continue;
        }

        let enemy_pos = enemy_transform.translation.truncate();
        let distance = player_pos.distance(enemy_pos);

        if distance <= player.radius + enemy.radius {
            next_state.set(crate::GameState::GameOver);
            break;
        }
    }
}

pub fn shield_orb_enemy_collision_system(
    mut commands: Commands,
    orb_query: Query<(Entity, &Transform, &ShieldOrb)>,
    enemy_query: Query<(Entity, &Transform, &Enemy)>,
    mut score: ResMut<crate::Score>,
) {
    let mut hit_enemies = std::collections::HashSet::new();

    for (orb_entity, orb_transform, orb) in orb_query.iter() {
        let orb_pos = orb_transform.translation.truncate();

        for (enemy_entity, enemy_transform, enemy) in enemy_query.iter() {
            if hit_enemies.contains(&enemy_entity) || !enemy.can_be_damaged() {
                continue;
            }

            let distance = orb_pos.distance(enemy_transform.translation.truncate());
            if distance <= orb.radius + enemy.radius {
                commands.entity(enemy_entity).despawn();
                commands.entity(orb_entity).despawn();
                hit_enemies.insert(enemy_entity);
                score.value += 1;
                break;
            }
        }
    }
}

pub fn enemy_movement_system(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Transform, &mut Enemy, &mut Sprite), Without<Player>>,
) {
    let player_transform = player_query.single().unwrap();
    let player_pos = player_transform.translation.truncate();

    for (mut transform, mut enemy, mut sprite) in enemy_query.iter_mut() {
        if let Some(timer) = &mut enemy.spawn_warning {
            timer.tick(time.delta());
            if !timer.is_finished() {
                continue;
            }
            enemy.spawn_warning = None;
            sprite.color = Color::srgb(0.8, 0.2, 0.2);
        }

        let enemy_pos = transform.translation.truncate();
        let offset = player_pos - enemy_pos;
        if offset.length_squared() <= f32::EPSILON {
            continue;
        }
        let direction = offset.normalize();

        transform.translation.x += direction.x * enemy.speed * time.delta_secs();
        transform.translation.y += direction.y * enemy.speed * time.delta_secs();
    }
}

pub fn position_enemy_spawn_system(
    mut commands: Commands,
    time: Res<Time<Fixed>>,
    mut spawn_timer: ResMut<PositionEnemySpawnTimer>,
    player_query: Query<&Transform, With<Player>>,
    position_enemies: Query<(), With<PositionSpawnedEnemy>>,
) {
    if !position_enemies.is_empty() {
        return;
    }

    spawn_timer.0.tick(time.delta());
    if !spawn_timer.0.is_finished() {
        return;
    }

    let player_transform = player_query.single().unwrap();
    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.65, 0.1), Vec2::splat(40.0)),
        Transform::from_xyz(
            player_transform.translation.x,
            player_transform.translation.y,
            0.05,
        ),
        Enemy {
            spawn_warning: Some(Timer::from_seconds(1.0, TimerMode::Once)),
            ..default()
        },
        PositionSpawnedEnemy,
    ));
    spawn_timer.0.reset();
}

pub fn projectile_enemy_collision_system(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform, &Projectile)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy)>,
    mut score: ResMut<crate::Score>,
) {
    let mut dead_enemies = std::collections::HashSet::new();

    for (projectile_entity, projectile_transform, projectile) in projectile_query.iter() {
        if !projectile.from_player {
            continue;
        }

        for (enemy_entity, enemy_transform, mut enemy) in enemy_query.iter_mut() {
            if dead_enemies.contains(&enemy_entity) || !enemy.can_be_damaged() {
                continue;
            }

            let distance = projectile_transform
                .translation
                .truncate()
                .distance(enemy_transform.translation.truncate());
            if distance <= enemy.radius + projectile.radius {
                enemy.health -= projectile.damage;
                if enemy.health <= 0 {
                    commands.entity(enemy_entity).despawn();
                    dead_enemies.insert(enemy_entity);
                    score.value += 1;
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
