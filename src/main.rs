use bevy::prelude::*;

#[derive(Component)]
struct GameCamera;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, hello_world)
    .run();
}

fn hello_world(mut commands: Commands) {
  println!("hello world!");
  commands.spawn((Camera2d::default(), GameCamera));
}
