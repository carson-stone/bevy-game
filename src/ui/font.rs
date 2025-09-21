use bevy::{color::palettes::css::GOLD, prelude::*};

#[derive(Resource)]
pub struct UiFont(pub Handle<Font>);

pub fn load_ui_font(mut commands: Commands, asset_server: Res<AssetServer>) {
  let font_handle: Handle<Font> = asset_server.load("fonts/font.ttf");

  // we can store the handle in a resource:
  //  - to prevent the asset from being unloaded
  //  - if we want to use it to access the asset later
  commands.insert_resource(UiFont(font_handle));
}

pub fn write_text(mut commands: Commands, font_handle: Res<UiFont>) {
  // add text to screen
  commands.spawn((
    Text::new("hello\nbevy!"),
    TextFont {
      // This font is loaded and will be used instead of the default font.
      font: font_handle.0.clone(),
      font_size: 36.0,
      ..default()
    },
    TextColor(GOLD.into()),
    Node {
      position_type: PositionType::Absolute,
      top: Val::Px(5.0),
      left: Val::Px(5.0),
      ..default()
    },
  ));
}
