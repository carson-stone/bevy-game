use super::health::Health;
use super::velocity::Velocity;
use super::wall::*;
use bevy::prelude::*;

pub const ENEMY_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const ENEMY_SPEED: f32 = 75.0;
// How close can the enemy get to the wall
const ENEMY_PADDING: f32 = 0.0;

#[derive(Component)]
#[require(Sprite, Transform, super::Collider, Health)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
  pub marker: Enemy,
  // We can nest/include another bundle.
  // Add the components for a standard Bevy Sprite:
  pub sprite: Sprite,
  pub transform: Transform,
  pub collider: super::Collider,
  pub health: Health,
}

impl Default for EnemyBundle {
  fn default() -> Self {
    Self {
      marker: Enemy,
      sprite: Sprite::from_color(Color::srgb(1.2, 1.2, 5.5), Vec2::ONE),
      transform: Transform {
        translation: Vec2::ZERO.extend(0.0),
        scale: ENEMY_SIZE.extend(1.0),
        ..default()
      },
      collider: super::Collider::default(),
      health: Health(100),
    }
  }
}

pub fn apply_enemy_ai(enemy_query: Single<(&mut Velocity, &mut Transform), With<Enemy>>) {
  let (mut velocity, transform) = enemy_query.into_inner();

  let should_move_down = transform.translation.y > 0.0;

  let mut direction = Vec2::ZERO;
  direction.y = if should_move_down { -1.0 } else { 1.0 };

  velocity.y = direction.y * ENEMY_SPEED;
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
