use bevy::prelude::*;

const WALL_THICKNESS: f32 = 10.0;
// x coordinates
const LEFT_WALL: f32 = -450.0;
const RIGHT_WALL: f32 = 450.0;
// y coordinates
const BOTTOM_WALL: f32 = -300.0;
const TOP_WALL: f32 = 300.0;

pub enum AreaLocation {
  Left,
  Right,
  Bottom,
  Top,
}

impl AreaLocation {
  /// Location of the *center* of the wall, used in `transform.translation()`
  fn position(&self) -> Vec2 {
    match self {
      AreaLocation::Left => Vec2::new(LEFT_WALL, 0.),
      AreaLocation::Right => Vec2::new(RIGHT_WALL, 0.),
      AreaLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
      AreaLocation::Top => Vec2::new(0., TOP_WALL),
    }
  }

  /// (x, y) dimensions of the wall, used in `transform.scale()`
  fn size(&self) -> Vec2 {
    let arena_height = TOP_WALL - BOTTOM_WALL;
    let arena_width = RIGHT_WALL - LEFT_WALL;
    // Make sure we haven't messed up our constants
    assert!(arena_height > 0.0);
    assert!(arena_width > 0.0);

    match self {
      AreaLocation::Left | AreaLocation::Right => {
        Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
      }
      AreaLocation::Bottom | AreaLocation::Top => {
        Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
      }
    }
  }
}

#[derive(Component)]
#[require(Sprite, Transform, super::Collider)]
pub struct Wall;

impl Wall {
  pub fn new(area_location: AreaLocation) -> (Wall, Sprite, Transform, super::Collider) {
    (
      Wall,
      Sprite::from_color(Color::srgb(0.8, 0.8, 0.8), Vec2::ONE),
      Transform {
        // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
        // This is used to determine the order of our sprites
        translation: area_location.position().extend(0.0),
        // The z-scale of 2D objects must always be 1.0,
        // or their ordering will be affected in surprising ways.
        scale: area_location.size().extend(1.0),
        ..default()
      },
      super::Collider::default(),
    )
  }
}
