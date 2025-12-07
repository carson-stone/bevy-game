mod collisions;
mod enemy;
mod health;
mod player;
mod utils;
mod velocity;
mod wall;

use bevy::prelude::*;
use enemy::{ENEMY_SIZE, Enemy, EnemyBundle};
use health::Health;
use player::{Player, PlayerBundle};
use wall::{AreaLocation, Wall};

pub use collisions::{CollisionEvent, check_for_collisions, debug_collisions};
pub use enemy::{apply_enemy_ai, move_enemy};
pub use player::{move_player, update_player_velocity};
pub use utils::Direction;

pub fn build_world(mut commands: Commands) {
  // area walls
  commands.spawn(Wall::new(AreaLocation::Top));
  commands.spawn(Wall::new(AreaLocation::Bottom));
  commands.spawn(Wall::new(AreaLocation::Left));
  commands.spawn(Wall::new(AreaLocation::Right));

  // player
  commands.spawn(PlayerBundle::default());

  // enemy
  commands.spawn(EnemyBundle {
    transform: Transform {
      translation: Vec2::new(0.0, 100.0).extend(0.0),
      scale: ENEMY_SIZE.extend(1.0),
      ..default()
    },
    ..default()
  });
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameplaySet {
  Player,
  Enemy,
}
