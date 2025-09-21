mod camera;
mod ui;

use bevy::{app::PluginGroupBuilder, prelude::*};

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

fn setup(mut commands: Commands) {
  info!("setting up game");

  // setup camera
  commands.spawn((Camera2d::default(), camera::GameCamera));
}
