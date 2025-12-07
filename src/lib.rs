mod camera;
mod gameplay;
mod input;
mod ui;

use bevy::{app::PluginGroupBuilder, prelude::*};
use camera::CameraPlugin;
use gameplay::{CollisionEvent, GameplaySet};
use input::{InputSet, MovePlayerEvent};
use ui::UiPlugin;

struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, setup)
      .add_systems(
        Update,
        (
          (input::handle_player_input)
            .in_set(InputSet)
            .before(GameplaySet::Player),
          (gameplay::update_player_velocity, gameplay::move_player)
            .chain()
            .in_set(GameplaySet::Player),
          (gameplay::apply_enemy_ai, gameplay::move_enemy)
            .chain()
            .in_set(GameplaySet::Enemy)
            .after(GameplaySet::Player),
          (gameplay::check_for_collisions, gameplay::debug_collisions).chain(),
        ),
      )
      .add_event::<MovePlayerEvent>()
      .add_event::<CollisionEvent>();
  }
}

pub struct GamePluginGroup;

impl PluginGroup for GamePluginGroup {
  fn build(self) -> PluginGroupBuilder {
    PluginGroupBuilder::start::<Self>()
      .add(GamePlugin)
      .add(CameraPlugin)
      .add(UiPlugin)
  }
}

fn setup(commands: Commands) {
  info!("setting up game");

  gameplay::build_world(commands);
}
