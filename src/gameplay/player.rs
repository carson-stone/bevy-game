use super::health::Health;
use super::velocity::Velocity;
use super::wall::*;
use bevy::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const PLAYER_SPEED: f32 = 200.0;
// How close can the player get to the wall
const PLAYER_PADDING: f32 = 0.0;

#[derive(Component)]
#[require(Sprite, Transform, super::Collider, Health, Velocity)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
  marker: Player,
  // We can nest/include another bundle.
  // Add the components for a standard Bevy Sprite:
  sprite: Sprite,
  transform: Transform,
  collider: super::Collider,
  health: Health,
  velocity: Velocity,
}

impl Default for PlayerBundle {
  fn default() -> Self {
    Self {
      marker: Player,
      sprite: Sprite::from_color(Color::srgb(1.0, 1.0, 0.5), Vec2::ONE),
      transform: Transform {
        translation: Vec2::ZERO.extend(0.0),
        scale: PLAYER_SIZE.extend(1.0),
        ..default()
      },
      collider: super::Collider::default(),
      health: Health(100),
      velocity: Velocity::zero(),
    }
  }
}

pub fn update_player_velocity(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut player_velocity: Single<&mut Velocity, With<Player>>,
) {
  let mut direction = Vec2::ZERO;

  if keyboard_input.pressed(KeyCode::ArrowUp) {
    direction.y = 1.0;
  } else if keyboard_input.pressed(KeyCode::ArrowRight) {
    direction.x = 1.0;
  } else if keyboard_input.pressed(KeyCode::ArrowDown) {
    direction.y = -1.0;
  } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
    direction.x = -1.0;
  }

  player_velocity.x = direction.x * PLAYER_SPEED;
  player_velocity.y = direction.y * PLAYER_SPEED;
}

pub fn move_player(
  player_query: Single<(&mut Transform, &mut Velocity), With<Player>>,
  time: Res<Time>,
) {
  let (mut transform, velocity) = player_query.into_inner();

  // Update the player position, making sure it doesn't cause the player to leave the area
  let top_bound = TOP_WALL - WALL_THICKNESS / 2.0 - PLAYER_SIZE.y / 2.0 - PLAYER_PADDING;
  let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PLAYER_SIZE.x / 2.0 - PLAYER_PADDING;
  let bottom_bound = BOTTOM_WALL + WALL_THICKNESS / 2.0 + PLAYER_SIZE.y / 2.0 + PLAYER_PADDING;
  let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PLAYER_SIZE.x / 2.0 + PLAYER_PADDING;

  // update the transform (adjustments may be applied by other systems afterward)
  transform.translation.x =
    (transform.translation.x + velocity.x * time.delta_secs()).clamp(left_bound, right_bound);
  transform.translation.y =
    (transform.translation.y + velocity.y * time.delta_secs()).clamp(bottom_bound, top_bound);
}
