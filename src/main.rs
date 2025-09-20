use bevy::prelude::*;
use bevy_game::GamePluginGroup;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(GamePluginGroup)
    .run();
}
