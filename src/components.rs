use bevy::prelude::*;

use crate::utils::{RIGHT_WALL, BOTTOM_WALL, TOP_WALL, WALL_THICKNESS, LEFT_WALL, WALL_COLOR};


#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Ball;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider;

#[derive(Event, Default)]
pub struct CollisionEvent;

#[derive(Component)]
pub struct Brick;

#[derive(Resource)]
pub struct CollisionSound(pub Handle<AudioSource>);

// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
pub struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    pub sprite_bundle: SpriteBundle,
    pub collider: Collider,
}

/// Which side of the arena is this wall located on?
pub enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

// This resource tracks the game's score
#[derive(Resource)]
pub struct Scoreboard {
    pub player_1_score: usize,
    pub player_2_score: usize,
}

impl WallLocation {
	pub fn position(&self) -> Vec2 {
			match self {
					WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
					WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
					WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
					WallLocation::Top => Vec2::new(0., TOP_WALL),
			}
	}

	pub fn size(&self) -> Vec2 {
			let arena_height = TOP_WALL - BOTTOM_WALL;
			let arena_width = RIGHT_WALL - LEFT_WALL;
			// Make sure we haven't messed up our constants
			assert!(arena_height > 0.0);
			assert!(arena_width > 0.0);

			match self {
					WallLocation::Left | WallLocation::Right => {
							Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
					}
					WallLocation::Bottom | WallLocation::Top => {
							Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
					}
			}
	}
}

impl WallBundle {
	// This "builder method" allows us to reuse logic across our wall entities,
	// making our code easier to read and less prone to bugs when we change the logic
	pub fn new(location: WallLocation) -> WallBundle {
			WallBundle {
					sprite_bundle: SpriteBundle {
							transform: Transform {
									// We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
									// This is used to determine the order of our sprites
									translation: location.position().extend(0.0),
									// The z-scale of 2D objects must always be 1.0,
									// or their ordering will be affected in surprising ways.
									// See https://github.com/bevyengine/bevy/issues/4149
									scale: location.size().extend(1.0),
									..default()
							},
							sprite: Sprite {
									color: WALL_COLOR,
									..default()
							},
							..default()
					},
					collider: Collider,
			}
	}
}

// OURS
#[derive(Component)]
pub struct Player1;


#[derive(Component)]
pub struct Player2;