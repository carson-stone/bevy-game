use bevy::{log::LogPlugin, prelude::*};
use bevy_game::GamePluginGroup;

fn main() {
  let mut app = App::new();

  // configure plugins
  app.add_plugins(GamePluginGroup);

  #[cfg(debug_assertions)]
  app.add_plugins(DefaultPlugins.set(LogPlugin {
    filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy_game=debug".into(),
    level: bevy::log::Level::DEBUG,
    ..default()
  }));

  // run the app
  app.run();
}
