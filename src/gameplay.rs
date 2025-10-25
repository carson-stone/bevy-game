mod wall;

use bevy::prelude::*;
use wall::Wall;

#[derive(Component, Default)]
struct Collider;

pub fn build_world(mut commands: Commands) {
  commands.spawn(Wall::new());
}
