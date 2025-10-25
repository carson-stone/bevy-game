use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::linear_rgb(0.0, 0.0, 0.0);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    // set the global default clear color (background color for cameras)
    app.insert_resource(ClearColor(BACKGROUND_COLOR));

    // setup camera
    app.add_systems(Startup, setup_camera);
  }
}

#[derive(Component)]
#[require(Camera2d)]
pub struct GameCamera;

fn setup_camera(mut commands: Commands) {
  commands.spawn(GameCamera);
}
