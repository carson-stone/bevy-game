use super::health::Health;
use super::physics::{Collider, Velocity};
use super::player::Player;
use super::wall::*;
use bevy::prelude::*;

pub const ENEMY_SIZE: Vec2 = Vec2::new(30.0, 30.0);
// How close can the enemy get to the wall
const ENEMY_PADDING: f32 = 0.0;
const ENEMY_SPEED: f32 = 75.0;

#[derive(Component, Default)]
#[require(Sprite, Collider, Velocity, Health)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
  pub marker: Enemy,
  pub sprite: Sprite,
  pub collider: Collider,
  pub velocity: Velocity,
  pub transform: Transform,
  pub health: Health,
}

impl Default for EnemyBundle {
  fn default() -> Self {
    Self {
      marker: Enemy::default(),
      sprite: Sprite::from_color(Color::srgb(1.2, 1.2, 5.5), Vec2::ONE),
      collider: Collider::default(),
      velocity: Velocity { x: 0.0, y: 0.0 },
      transform: Transform {
        translation: Vec2::ZERO.extend(0.0),
        scale: ENEMY_SIZE.extend(1.0),
        ..default()
      },
      health: Health(100),
    }
  }
}

pub fn apply_enemy_ai(
  player_query: Single<&Transform, With<Player>>,
  enemy_query: Single<(&Transform, &mut Velocity), With<Enemy>>,
) {
  let player_transform = player_query.into_inner();
  let (enemy_transform, mut enemy_velocity) = enemy_query.into_inner();

  let mut direction = Vec2::ZERO;

  let x_diff = enemy_transform.translation.x - player_transform.translation.x;
  let y_diff = enemy_transform.translation.y - player_transform.translation.y;

  if x_diff.abs() > y_diff.abs() {
    direction.x = if x_diff < 0.0 { 1.0 } else { -1.0 };
  } else {
    direction.y = if y_diff < 0.0 { 1.0 } else { -1.0 };
  }

  enemy_velocity.x = direction.x * ENEMY_SPEED;
  enemy_velocity.y = direction.y * ENEMY_SPEED;
}

pub fn move_enemy(
  enemy_query: Single<(&mut Transform, &mut Velocity), With<Enemy>>,
  time: Res<Time>,
) {
  let (mut transform, velocity) = enemy_query.into_inner();

  // Update the enemy position, making sure it doesn't cause the enemy to leave the arena
  let top_bound = TOP_WALL - WALL_THICKNESS / 2.0 - ENEMY_SIZE.y / 2.0 - ENEMY_PADDING;
  let bottom_bound = BOTTOM_WALL + WALL_THICKNESS / 2.0 + ENEMY_SIZE.y / 2.0 + ENEMY_PADDING;

  transform.translation.y =
    (transform.translation.y + velocity.y * time.delta_secs()).clamp(bottom_bound, top_bound);
}
