mod camera;
mod gameplay;
mod ui;

use bevy::{app::PluginGroupBuilder, prelude::*};
use gameplay::{
  GameplaySet, apply_enemy_ai, build_world, check_for_collisions, move_enemy, move_player,
  update_player_velocity,
};

struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, setup).add_systems(
      Update,
      (
        (
          update_player_velocity.before(check_for_collisions),
          move_player,
        )
          .in_set(GameplaySet::Player),
        (
          (apply_enemy_ai.before(move_enemy)),
          (move_enemy).before(check_for_collisions),
        )
          .in_set(GameplaySet::Enemy),
        check_for_collisions,
      ),
    );
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

fn setup(commands: Commands) {
  info!("setting up game");

  build_world(commands);
}
