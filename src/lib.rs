mod camera;
mod gameplay;
mod ui;

use bevy::{app::PluginGroupBuilder, prelude::*};
use gameplay::build_world;

struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, setup);
  }
}

pub struct GamePluginGroup;

impl PluginGroup for GamePluginGroup {
  fn build(self) -> PluginGroupBuilder {
    PluginGroupBuilder::start::<Self>()
      .add(GamePlugin)
      .add(camera::CameraPlugin)
      .add(ui::UiPlugin)
  }
}

fn setup(commands: Commands) {
  info!("setting up game");

  build_world(commands);
}
