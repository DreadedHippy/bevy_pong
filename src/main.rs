use bevy::prelude::*;
use plugins::HelloPlugin;

mod entities;
mod components;
mod systems;
mod plugins;
mod resources;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        // .add_systems(Startup, system_add_people)
        // .add_systems(Update, (system_hello_world, systems_greet_people))
        .run();
}


