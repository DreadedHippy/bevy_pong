
use bevy::prelude::*;


use crate::{components::{Person, Name, Player, Velocity, Position}, resources::GreetTimer};
const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const PLAYER_INITIAL_POSITION: Vec3 = Vec3::new(0.0, -240., 0.0);

pub fn system_setup(mut commands: Commands) {
	
    // Camera
    commands.spawn(Camera2dBundle::default());
}

pub fn system_add_people(mut commands: Commands) {
	commands.spawn((Person, Name("Elaina Proctor".to_string())));
	commands.spawn((Person, Name("Renzo Hume".to_string())));
	commands.spawn((Person, Name("Zayna Nieves".to_string())));
	commands.spawn((
		Player,
		Name("Player 1".to_string()),
		SpriteBundle {
			sprite: Sprite {
				color: Color::GREEN,
				..default()
			},
			transform: Transform {
				translation: PLAYER_INITIAL_POSITION,
				scale: PADDLE_SIZE,
				..default()
			},
			..default()
		},
		Velocity {
			x_velocity: 0.0,
			y_velocity: 0.0
		},
		Position {
			x_position: PLAYER_INITIAL_POSITION.x,
			y_position: PLAYER_INITIAL_POSITION.y
		}
	));
}

pub fn systems_add_player(mut commands: Commands) {
	// commands.spawn((
	// 	Player,
	// 	Name("Player 1".to_string()),
	// 	SpriteBundle {
	// 		sprite: Sprite {
	// 			color: Color::RED,
	// 			..default()
	// 		},
	// 		transform: Transform {
	// 			translation: Vec3::new(0.0, -240., 0.0),
	// 			..default()
	// 		},
	// 		..default()
	// 	},
	// ));
}

pub fn systems_greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
	//  update our timer with the time elapsed since the last update
	// if that causes our timer to finish, we say hello to everyone

	if timer.0.tick(time.delta()).just_finished() {
		for name in &query {
			println!("Hello {}!", name.0)
		}
	}
}

pub fn system_move_player(
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&mut Transform, &mut Velocity)>,
	time: Res<Time>
) {
	let mut player_params = query.single_mut();
	let mut direction = 0.0;

	// Accelerate forward on "Right arrow" press
	if keyboard_input.pressed(KeyCode::Left) {
		player_params.1.x_velocity -= 1.5;
	}

	// Decelerate / Accelerate backwards on "Left arrow" press
	if keyboard_input.pressed(KeyCode::Right) {
		player_params.1.x_velocity += 1.5;
	}

	if player_params.1.x_velocity != 0.0 {
		let speed_change = match  player_params.1.x_velocity > 0.0 {
			true => -0.5,
			false => 0.5
		};

		player_params.1.x_velocity += speed_change;
	}

	// Calculate the new player position based on user input
	let new_player_position = player_params.0.translation.x + player_params.1.x_velocity * time.delta_seconds();
	let (right_bound, left_bound) = (450., -450.);

	player_params.0.translation.x = new_player_position.clamp(left_bound, right_bound);
}