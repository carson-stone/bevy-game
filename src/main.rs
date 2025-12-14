use bevy::{log::LogPlugin, prelude::*};
use bevy_game::GamePluginGroup;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(LogPlugin {
      filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy_game=debug".into(),
      level: bevy::log::Level::DEBUG,
      ..default()
    }))
    .add_plugins(GamePluginGroup)
    .run();
}
