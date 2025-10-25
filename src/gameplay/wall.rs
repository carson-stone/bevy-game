use bevy::prelude::*;

#[derive(Component)]
#[require(Sprite, Transform, super::Collider)]
pub struct Wall;

impl Wall {
  pub fn new() -> (Wall, Sprite, Transform, super::Collider) {
    (
      Wall,
      Sprite::from_color(Color::srgb(0.8, 0.8, 0.8), Vec2::ONE),
      Transform {
        // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
        // This is used to determine the order of our sprites
        translation: Vec2::new(0.0, 0.0).extend(0.0),
        // The z-scale of 2D objects must always be 1.0,
        // or their ordering will be affected in surprising ways.
        scale: Vec2::new(50.0, 20.0).extend(1.0),
        ..default()
      },
      super::Collider::default(),
    )
  }
}
