use crate::gameplay::Direction;
use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputSet;

#[derive(Event)]
pub struct MovePlayerEvent(pub Direction);

pub fn handle_player_input(
  mut move_player_event_writer: EventWriter<MovePlayerEvent>,
  keyboard_input: Res<ButtonInput<KeyCode>>,
) {
  if keyboard_input.pressed(KeyCode::ArrowUp) {
    move_player_event_writer.write(MovePlayerEvent(Direction::Up));
  } else if keyboard_input.pressed(KeyCode::ArrowRight) {
    move_player_event_writer.write(MovePlayerEvent(Direction::Right));
  } else if keyboard_input.pressed(KeyCode::ArrowDown) {
    move_player_event_writer.write(MovePlayerEvent(Direction::Down));
  } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
    move_player_event_writer.write(MovePlayerEvent(Direction::Left));
  }
}
