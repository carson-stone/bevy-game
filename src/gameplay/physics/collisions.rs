pub mod collision_debugger;

use super::super::Wall;
use crate::gameplay::physics::Velocity;
use bevy::{
  math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
  prelude::*,
};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CollisionNormal {
  Left,
  Right,
  Top,
  Bottom,
}

#[derive(Event)]
pub struct CollisionEvent(pub (Entity, Entity), pub f32, pub CollisionNormal);

#[derive(Component, Default)]
pub struct Collider;

fn determine_collision(
  (mut box_a, velocity_a): (&Aabb2d, &Velocity),
  (mut box_b, velocity_b): (&Aabb2d, &Velocity),
) -> Option<(f32, CollisionNormal)> {
  // debug!("box A {:?}", &box_a);
  // debug!("box B {:?}", &box_b);

  // if !box_a.intersects(&box_b) {
  //   return None;
  // }

  // let closest_point = box_b.closest_point(box_a.center());
  // let offset = box_a.center() - closest_point;

  // let side = if offset.x.abs() > offset.y.abs() && offset.x < 0.0 {
  //   Collision::Left
  // } else if offset.x.abs() > offset.y.abs() {
  //   Collision::Right
  // } else if offset.y > 0.0 {
  //   Collision::Top
  // } else {
  //   Collision::Bottom
  // };

  // Some(side)

  let width_a = box_a.max.x - box_a.min.x;
  let height_a = box_a.max.y - box_a.min.y;
  let width_b = box_b.max.x - box_b.min.x;
  let height_b = box_b.max.y - box_b.min.y;
  // let minkowski_sum = box_b.grow(Vec2::new(width_a, height_a));
  // let point_a = box_a.min;

  // Use a relative velocity of Box A compared to Box B
  let relative_velocity_a = Vec2::new(velocity_a.x - velocity_b.x, velocity_a.y - velocity_b.y);

  // debug!("relative vel");
  // debug!("{}", &relative_velocity_a);

  let x_inv_entry: f32;
  let y_inv_entry: f32;
  let x_inv_exit: f32;
  let y_inv_exit: f32;

  if relative_velocity_a.x > 0.0 {
    x_inv_entry = box_b.min.x - width_a;
    x_inv_exit = width_b - box_a.min.x;
  } else {
    x_inv_entry = width_b - box_a.min.x;
    x_inv_exit = box_b.min.x - width_a;
  }

  if relative_velocity_a.y > 0.0 {
    y_inv_entry = box_b.min.y - height_a;
    y_inv_exit = height_b - box_a.min.y;
  } else {
    y_inv_entry = height_b - box_a.min.y;
    y_inv_exit = box_b.min.y - height_a;
  }

  None
}

pub fn check_for_collisions(
  mut collision_event_writer: EventWriter<CollisionEvent>,
  colliders_query: Query<(Entity, &Transform, &Velocity, Option<&Wall>), With<Collider>>,
) {
  // `iter_combinations` will iterate over all unique pairs of entities (skipping self-pairs)
  for [collider_a, collider_b] in colliders_query.iter_combinations() {
    let (entity_a, transform_a, velocity_a, maybe_wall_a) = collider_a;
    let (entity_b, transform_b, velocity_b, maybe_wall_b) = collider_b;

    // prevent collisions between walls
    if maybe_wall_a.is_some() && maybe_wall_b.is_some() {
      continue;
    }

    // check for a collision
    let collision = determine_collision(
      (
        &Aabb2d::new(
          transform_a.translation.truncate(),
          transform_a.scale.truncate() / 2.0,
        ),
        velocity_a,
      ),
      (
        &Aabb2d::new(
          transform_b.translation.truncate(),
          transform_b.scale.truncate() / 2.0,
        ),
        velocity_b,
      ),
    );

    // handle collision
    if let Some((time, normal)) = collision {
      // emit a collision event
      collision_event_writer.write(CollisionEvent((entity_a, entity_b), time, normal));
    }
  }
}
