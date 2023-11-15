use bevy::prelude::Component;


#[derive(Component)]
pub struct Position { pub x_position: f32, pub y_position: f32 }


#[derive(Component)]
pub struct Velocity { pub x_velocity: f32, pub y_velocity: f32 }


#[derive(Component)]
pub struct Person;


#[derive(Component)]
pub struct Player;


#[derive(Component)]
pub struct Name(pub String);