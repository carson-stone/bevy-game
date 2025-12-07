use super::collisions::Collider;
use super::health::Health;
use super::utils::Direction;
use super::velocity::Velocity;
use super::wall::*;
use crate::input::MovePlayerEvent;
use bevy::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::new(30.0, 30.0);
// How close can the player get to the wall
const PLAYER_PADDING: f32 = 0.0;
const PLAYER_SPEED: f32 = 200.0;

#[derive(Component, Default)]
#[require(Sprite, Collider, Velocity, Health)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
  marker: Player,
  sprite: Sprite,
  collider: Collider,
  velocity: Velocity,
  transform: Transform,
  health: Health,
}

impl Default for PlayerBundle {
  fn default() -> Self {
    Self {
      marker: Player::default(),
      sprite: Sprite::from_color(Color::srgb(1.0, 1.0, 0.5), Vec2::ONE),
      collider: Collider::default(),
      velocity: Velocity { x: 0.0, y: 0.0 },
      transform: Transform {
        translation: Vec2::ZERO.extend(0.0),
        scale: PLAYER_SIZE.extend(1.0),
        ..default()
      },
      health: Health(100),
    }
  }
}

pub fn update_player_velocity(
  mut move_player_event_reader: EventReader<MovePlayerEvent>,
  mut player_velocity: Single<&mut Velocity, With<Player>>,
) {
  let mut new_velocity = Vec2::ZERO;

  for event in move_player_event_reader.read() {
    let direction = &event.0;

    match direction {
      Direction::Up => new_velocity.y = 1.0,
      Direction::Right => new_velocity.x = 1.0,
      Direction::Down => new_velocity.y = -1.0,
      Direction::Left => new_velocity.x = -1.0,
    }
  }

  player_velocity.x = new_velocity.x * PLAYER_SPEED;
  player_velocity.y = new_velocity.y * PLAYER_SPEED;
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
