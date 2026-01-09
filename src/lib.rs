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
    // add systems to schedules and sets and register events
    app
      // Startup
      .add_systems(Startup, setup)
      // Update
      .add_systems(
        Update,
        (
          (input::handle_player_input).in_set(InputSet),
          (
            gameplay::update_player_velocity,
            gameplay::adjust_player_velocity_when_collision,
            gameplay::move_player,
          )
            .chain()
            .in_set(GameplaySet::Player),
          // (gameplay::apply_enemy_ai, gameplay::move_enemy)
          //   .chain()
          //   .in_set(GameplaySet::Enemy),
          (
            gameplay::check_for_collisions,
            gameplay::debug_collisions,
            gameplay::debug_collision_events,
          )
            .chain(),
        ),
      )
      // events
      .add_event::<MovePlayerEvent>()
      .add_event::<CollisionEvent>();

    // configure system sets
    app.configure_sets(
      Update,
      (
        InputSet.before(GameplaySet::Player),
        GameplaySet::Enemy.before(GameplaySet::Player),
      ),
    );
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
