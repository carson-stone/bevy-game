use super::{Collider, CollisionEvent, Velocity};
use bevy::prelude::*;

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
///   - ray from origin of entity being debugged to point of collision on target.
///   - normal vector at point of collision
pub fn debug_collisions(
  colliders_query: Query<
    (Entity, &Transform, &Velocity, &CollisionDebugger),
    With<CollisionDebugger>,
  >,
) {
  for (entity, transform, velocity, debugger) in colliders_query.iter() {
    debug!("Entity: {:?}, Target: {:?}", entity, debugger.target_entity);
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
