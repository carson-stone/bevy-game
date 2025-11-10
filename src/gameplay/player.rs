use bevy::prelude::*;

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
