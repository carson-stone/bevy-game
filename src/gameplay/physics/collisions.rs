use super::super::Wall;
use bevy::{
  math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
  prelude::*,
};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Collision {
  Left,
  Right,
  Top,
  Bottom,
}

#[derive(Event)]
pub struct CollisionEvent(pub Entity, pub Entity, pub Collision);

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
  mut collision_event_writer: EventWriter<CollisionEvent>,
  collider_query_a: Query<(Entity, &Transform, Option<&Wall>), With<Collider>>,
  collider_query_b: Query<(Entity, &Transform, Option<&Wall>), With<Collider>>,
) {
  let mut entities_colliding: HashSet<(Entity, Entity)> = HashSet::new();

  for (entity_a, transform_a, maybe_wall_a) in &collider_query_a {
    for (entity_b, transform_b, maybe_wall_b) in &collider_query_b {
      // prevent collisions between a transform and itself
      if entity_a == entity_b {
        continue;
      }

      // prevent collisions between walls
      if maybe_wall_a.is_some() && maybe_wall_b.is_some() {
        continue;
      }

      // prevent double-processing of collisions
      if entities_colliding.contains(&(entity_a, entity_b))
        || entities_colliding.contains(&(entity_b, entity_a))
      {
        continue;
      }

      // check for a collision
      let collision = determine_collision(
        Aabb2d::new(
          transform_a.translation.truncate(),
          transform_a.scale.truncate() / 2.0,
        ),
        Aabb2d::new(
          transform_b.translation.truncate(),
          transform_b.scale.truncate() / 2.0,
        ),
      );

      // handle collision
      if collision.is_some() {
        // keep track of collisions so we don't double-process them
        entities_colliding.insert((entity_a, entity_b));
        entities_colliding.insert((entity_b, entity_a));

        // emit a collision event
        collision_event_writer.write(CollisionEvent(entity_a, entity_b, collision.unwrap()));
      }
    }
  }
}

pub fn debug_collisions(mut collision_event_reader: EventReader<CollisionEvent>) {
  for event in collision_event_reader.read() {
    eprintln!(
      "Entity {:?} collided with {:?} on side {:?}",
      event.0, event.1, event.2
    );
  }
}
