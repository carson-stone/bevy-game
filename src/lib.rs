mod camera;
mod ui;

use bevy::{app::PluginGroupBuilder, color::palettes::css::GOLD, prelude::*};
use ui::font;

struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    // set the global default clear color (background color for cameras)
    app
      .insert_resource(ClearColor(Color::linear_rgb(0.9, 0.3, 0.6)))
      .add_systems(Startup, (setup, (write_text)).chain());
  }
}

pub struct GamePluginGroup;

impl PluginGroup for GamePluginGroup {
  fn build(self) -> PluginGroupBuilder {
    PluginGroupBuilder::start::<Self>()
      .add(GamePlugin)
      .add(ui::UiPlugin)
  }
}

fn setup(mut commands: Commands) {
  info!("setting up game");

  // setup camera
  commands.spawn((Camera2d::default(), camera::GameCamera));
}

fn write_text(mut commands: Commands, font_handle: Res<font::UiFont>) {
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
