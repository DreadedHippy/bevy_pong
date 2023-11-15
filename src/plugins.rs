use bevy::prelude::*;

use crate::{systems::{system_add_people, systems_greet_people, systems_add_player, system_setup, system_move_player}, resources::GreetTimer};

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
			.insert_resource(ClearColor(Color::BLACK))
			.add_systems(Startup, (system_setup, system_add_people))
			.add_systems(Update, systems_greet_people)
			.add_systems(Update, system_move_player);
	}
}