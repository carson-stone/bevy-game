use bevy::prelude::*;

#[derive(Resource)]
pub struct UiFont(pub Handle<Font>);

pub fn load_ui_font(mut commands: Commands, asset_server: Res<AssetServer>) {
  let handle: Handle<Font> = asset_server.load("fonts/font.ttf");

  // we can store the handle in a resource:
  //  - to prevent the asset from being unloaded
  //  - if we want to use it to access the asset later
  commands.insert_resource(UiFont(handle));
}
