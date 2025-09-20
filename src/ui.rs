pub mod font;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, font::load_ui_font);
  }
}
