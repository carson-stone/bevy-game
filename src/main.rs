use bevy::{app::PluginGroupBuilder, color::palettes::css::GOLD, prelude::*};

#[derive(Resource)]
struct UiFont(Handle<Font>);

#[derive(Component)]
struct GameCamera;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(GamePluginGroup)
    .run();
}

fn setup(mut commands: Commands) {
  println!("setting up game");

  // setup camera
  commands.spawn((Camera2d::default(), GameCamera));
}

fn load_ui_font(mut commands: Commands, asset_server: Res<AssetServer>) {
  let handle: Handle<Font> = asset_server.load("fonts/font.ttf");

  // we can store the handle in a resource:
  //  - to prevent the asset from being unloaded
  //  - if we want to use it to access the asset later
  commands.insert_resource(UiFont(handle));
}

fn write_text(mut commands: Commands, font_handle: Res<UiFont>) {
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

struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    // set the global default clear color (background color for cameras)
    app
      .insert_resource(ClearColor(Color::linear_rgb(0.9, 0.3, 0.6)))
      .add_systems(Startup, (load_ui_font, setup, (write_text)).chain());
  }
}

struct GamePluginGroup;

impl PluginGroup for GamePluginGroup {
  fn build(self) -> PluginGroupBuilder {
    PluginGroupBuilder::start::<Self>().add(GamePlugin)
  }
}
