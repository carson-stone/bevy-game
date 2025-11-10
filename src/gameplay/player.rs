use super::wall::*;
use bevy::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const PLAYER_SPEED: f32 = 200.0;
// How close can the player get to the wall
const PLAYER_PADDING: f32 = 0.0;

#[derive(Component, Default)]
pub struct Health(u8);

#[derive(Component)]
struct Velocity;

#[derive(Component)]
#[require(Sprite, Transform, super::Collider, Health)]
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
}

impl PlayerBundle {
  pub fn new() -> Self {
    Self {
      marker: Player,
      sprite: Sprite::from_color(Color::srgb(1.0, 1.0, 0.5), Vec2::ONE),
      transform: Transform {
        translation: Vec2::new(0.0, 0.0).extend(0.0),
        scale: Vec2::new(30.0, 30.0).extend(1.0),
        ..default()
      },
      collider: super::Collider::default(),
      health: Health(100),
    }
  }
}

pub fn move_player(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut player_transform: Single<&mut Transform, With<Player>>,
  time: Res<Time>,
) {
  let mut direction = Vec2::new(0.0, 0.0);

  if keyboard_input.pressed(KeyCode::ArrowUp) {
    direction.y = 1.0;
  } else if keyboard_input.pressed(KeyCode::ArrowRight) {
    direction.x = 1.0;
  } else if keyboard_input.pressed(KeyCode::ArrowDown) {
    direction.y = -1.0;
  } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
    direction.x = -1.0;
  }

  // Update the player position, making sure it doesn't cause the player to leave the arena
  let top_bound = TOP_WALL - WALL_THICKNESS / 2.0 - PLAYER_SIZE.y / 2.0 - PLAYER_PADDING;
  let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PLAYER_SIZE.x / 2.0 - PLAYER_PADDING;
  let bottom_bound = BOTTOM_WALL + WALL_THICKNESS / 2.0 + PLAYER_SIZE.y / 2.0 + PLAYER_PADDING;
  let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PLAYER_SIZE.x / 2.0 + PLAYER_PADDING;

  player_transform.translation.x = (player_transform.translation.x
    + direction.x * PLAYER_SPEED * time.delta_secs())
  .clamp(left_bound, right_bound);
  player_transform.translation.y = (player_transform.translation.y
    + direction.y * PLAYER_SPEED * time.delta_secs())
  .clamp(bottom_bound, top_bound);
}
