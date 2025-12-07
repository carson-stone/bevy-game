use bevy::prelude::*;

#[derive(Component)]
#[require(Transform)]
pub struct Velocity {
  pub x: f32,
  pub y: f32,
}

impl Default for Velocity {
  fn default() -> Self {
    Self { x: 0.0, y: 0.0 }
  }
}
