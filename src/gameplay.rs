mod player;
mod wall;

use bevy::prelude::*;
use player::PlayerBundle;
use wall::{AreaLocation, Wall};

#[derive(Component, Default)]
struct Collider;

pub fn build_world(mut commands: Commands) {
  // area walls
  commands.spawn(Wall::new(AreaLocation::Top));
  commands.spawn(Wall::new(AreaLocation::Bottom));
  commands.spawn(Wall::new(AreaLocation::Left));
  commands.spawn(Wall::new(AreaLocation::Right));

  // player
  commands.spawn(PlayerBundle::new());
}
