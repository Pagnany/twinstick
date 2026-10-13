use bevy::prelude::*;
use rand::RngExt;

#[derive(Component)]
pub struct Obstacle {
    velocity: Vec2,
}

const OBSTACLE_COUNT: usize = 4;
const OBSTACLE_SIZE: f32 = 72.0;
const OBSTACLE_SPEED: f32 = 150.0;

pub fn obstacle_player_collision_system(
    player_query: Query<(&Transform, &crate::player::Player)>,
    obstacle_query: Query<(&Transform, &Obstacle)>,
    mut next_state: ResMut<NextState<crate::GameState>>,
) {
    let (player_transform, player) = player_query.single().unwrap();
    let player_pos = player_transform.translation.truncate();

    for (obstacle_transform, _) in &obstacle_query {
        let obstacle_pos = obstacle_transform.translation.truncate();
        if player_pos.distance(obstacle_pos) <= player.radius + OBSTACLE_SIZE / 2.0 {
            next_state.set(crate::GameState::GameOver);
            break;
        }
    }
}

pub fn obstacle_spawn_system(mut commands: Commands, obstacles: Query<(), With<Obstacle>>) {
    let count = obstacles.iter().count();
    if count >= OBSTACLE_COUNT {
        return;
    }

    let mut rng = rand::rngs::ThreadRng::default();
    for _ in count..OBSTACLE_COUNT {
        let side = rng.random_range(0..=3);
        let drift = rng.random_range(-0.5..=0.5);
        let (x, y, direction) = match side {
            0 => (
                -crate::WINDOW_WIDTH / 2.0 - OBSTACLE_SIZE / 2.0,
                rng.random_range(-crate::WINDOW_HEIGHT / 2.0..=crate::WINDOW_HEIGHT / 2.0),
                Vec2::new(1.0, drift).normalize(),
            ),
            1 => (
                crate::WINDOW_WIDTH / 2.0 + OBSTACLE_SIZE / 2.0,
                rng.random_range(-crate::WINDOW_HEIGHT / 2.0..=crate::WINDOW_HEIGHT / 2.0),
                Vec2::new(-1.0, drift).normalize(),
            ),
            2 => (
                rng.random_range(-crate::WINDOW_WIDTH / 2.0..=crate::WINDOW_WIDTH / 2.0),
                -crate::WINDOW_HEIGHT / 2.0 - OBSTACLE_SIZE / 2.0,
                Vec2::new(drift, 1.0).normalize(),
            ),
            _ => (
                rng.random_range(-crate::WINDOW_WIDTH / 2.0..=crate::WINDOW_WIDTH / 2.0),
                crate::WINDOW_HEIGHT / 2.0 + OBSTACLE_SIZE / 2.0,
                Vec2::new(drift, -1.0).normalize(),
            ),
        };

        commands.spawn((
            Sprite::from_color(Color::srgb(0.2, 0.8, 0.2), Vec2::splat(OBSTACLE_SIZE)),
            Transform::from_xyz(x, y, -0.1),
            Obstacle {
                velocity: direction * OBSTACLE_SPEED,
            },
        ));
    }
}

pub fn obstacle_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut obstacles: Query<(Entity, &mut Transform, &Obstacle)>,
) {
    let half_width = crate::WINDOW_WIDTH / 2.0 + OBSTACLE_SIZE / 2.0;
    let half_height = crate::WINDOW_HEIGHT / 2.0 + OBSTACLE_SIZE / 2.0;

    for (entity, mut transform, obstacle) in &mut obstacles {
        transform.translation.x += obstacle.velocity.x * time.delta_secs();
        transform.translation.y += obstacle.velocity.y * time.delta_secs();

        if transform.translation.x < -half_width
            || transform.translation.x > half_width
            || transform.translation.y < -half_height
            || transform.translation.y > half_height
        {
            commands.entity(entity).despawn();
        }
    }
}
