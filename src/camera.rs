use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::linear_rgb(0.0, 0.0, 0.0);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    // set the global default clear color (background color for cameras)
    app.insert_resource(ClearColor(BACKGROUND_COLOR));
  }
}

#[derive(Component)]
pub struct GameCamera;
