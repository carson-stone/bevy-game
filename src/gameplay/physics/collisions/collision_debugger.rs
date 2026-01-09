use super::{Collider, CollisionEvent, Velocity};
use bevy::{color::palettes::css::*, math::Isometry2d, prelude::*};

// component that enables detailed collision debugging between two entities
#[derive(Component, Default)]
// #[require(Sprite)]
pub struct CollisionDebugger {
  pub target_entity: Option<Entity>,
}

// impl Default for CollisionDebugger {

// }

/// Detailed debugging of collision detection between entities that have a `CollisionDebugger`.
/// The debugger contains a reference to a target to debug potential collisions with.
///
/// 1. Set the velocity of both entities to 0.
/// 2. Show UI controls to simulate the velocity of both entities, while keeping the transforms
/// of both entities static.
/// 3. Show AABBs of entities.
/// 4. Show results:
///   - time to collision
///   - line from origin of entity being debugged to point of collision on target.
///   - ray that is the normal vector at the point of collision
pub fn debug_collisions(
  mut gizmos: Gizmos,
  debugger_query: Query<
    (Entity, &Transform, &Velocity, &CollisionDebugger),
    With<CollisionDebugger>,
  >,
  colliders_query: Query<&Transform, With<Collider>>,
) {
  // For entities that have debuggers which have `Some` target entity, use the bevy-provided `gizmos`
  // tool to add lines for visual debugging.
  for (entity, transform, velocity, debugger) in debugger_query
    .iter()
    .filter(|to_debug| to_debug.3.target_entity.is_some())
  {
    if debugger.target_entity.is_none() {
      continue;
    }

    let target_entity = debugger.target_entity.unwrap();

    let target_result = colliders_query.get(target_entity);

    if !target_result.is_ok() {
      eprintln!(
        "Error debugging collisions: Entity {:?} has target {:?}, but target could not be retrieved from query",
        entity, target_entity
      );
      continue;
    }

    let target = target_result.ok().unwrap();

    gizmos.line_2d(
      Vec2::new(transform.translation.x, transform.translation.y),
      Vec2::new(target.translation.x, target.translation.y),
      RED,
    );
    // gizmos.ray_2d(Vec2::Y * sin_t_scaled, Vec2::splat(80.), LIME);
  }
}

// debugs collision events
pub fn debug_collision_events(mut collision_event_reader: EventReader<CollisionEvent>) {
  for event in collision_event_reader.read() {
    let entities = event.0;
    let time = event.1;
    let normal = event.1;
    let (entity_a, entity_b) = entities;
    debug!(
      "Entity {:?} collides with {:?} at time {:?} on side {:?}",
      entity_a, entity_b, time, normal
    );
  }
}
