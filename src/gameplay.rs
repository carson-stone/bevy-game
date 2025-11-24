mod enemy;
mod health;
mod player;
mod velocity;
mod wall;

use bevy::{
  math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
  prelude::*,
};
use enemy::{ENEMY_SIZE, Enemy, EnemyBundle};
use health::Health;
use player::{Player, PlayerBundle};
use std::cmp;
use wall::{AreaLocation, Wall};

pub use enemy::{apply_enemy_ai, move_enemy};
pub use player::{move_player, update_player_velocity};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
  Left,
  Right,
  Top,
  Bottom,
}

#[derive(Component, Default)]
pub struct Collider;

fn determine_collision(box_a: Aabb2d, box_b: Aabb2d) -> Option<Collision> {
  debug!("{:?}, {:?}", &box_a, &box_b);

  if !box_a.intersects(&box_b) {
    return None;
  }

  let closest_point = box_b.closest_point(box_a.center());
  let offset = box_a.center() - closest_point;

  let side = if offset.x.abs() > offset.y.abs() && offset.x < 0.0 {
    Collision::Left
  } else if offset.x.abs() > offset.y.abs() {
    Collision::Right
  } else if offset.y > 0.0 {
    Collision::Top
  } else {
    Collision::Bottom
  };

  Some(side)
}

pub fn check_for_collisions(
  player_query: Single<(&Transform, &mut Health), With<Player>>,
  collider_query: Query<(Entity, &Transform, Option<&Enemy>), With<Collider>>,
) {
  let (player_transform, mut player_health) = player_query.into_inner();

  info!("player");
  info!(?player_transform.translation);
  info!(?player_transform.scale);

  for (other_entity, other_transform, maybe_enemy) in &collider_query {
    if let Some(enemy) = maybe_enemy {
      info!("other");
      info!(?other_transform.translation);
      info!(?other_transform.scale);
    }
    let collision = determine_collision(
      Aabb2d::new(
        player_transform.translation.truncate(),
        player_transform.scale.truncate() / 2.0,
      ),
      Aabb2d::new(
        other_transform.translation.truncate(),
        other_transform.scale.truncate() / 2.0,
      ),
    );

    if let Some(collision) = collision {
      let enemy_damage = 10;
      player_health.0 = (cmp::max(player_health.0, enemy_damage) - enemy_damage);
      info!("update player health to");
      info!("{}", player_health.0);
    }
  }
}

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
