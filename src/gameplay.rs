mod wall;

use bevy::prelude::*;
use wall::{AreaLocation, Wall};

#[derive(Component, Default)]
struct Collider;

pub fn build_world(mut commands: Commands) {
  commands.spawn(Wall::new(AreaLocation::Top));
  commands.spawn(Wall::new(AreaLocation::Bottom));
  commands.spawn(Wall::new(AreaLocation::Left));
  commands.spawn(Wall::new(AreaLocation::Right));
}
