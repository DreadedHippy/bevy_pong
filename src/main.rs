mod components;
mod utils;


// ! A simplified implementation of the classic game "Pong".

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
};
use rand::seq::SliceRandom;
use components::{Scoreboard, CollisionSound, CollisionEvent, Collider, Velocity, Ball};
use utils::{WALL_THICKNESS, PADDLE_SIZE, PADDLE_PADDING, RIGHT_WALL, LEFT_WALL, PADDLE_SPEED, BACKGROUND_COLOR};

use crate::{utils::{SCORE_COLOR, SCOREBOARD_FONT_SIZE, TEXT_COLOR, INITIAL_BALL_DIRECTION, BALL_SPEED, BALL_STARTING_POSITION, BALL_SIZE, BALL_COLOR, GAP_BETWEEN_PADDLE_AND_FLOOR, BOTTOM_WALL, TOP_WALL, PLAYER_SIZE, PLAYER_1_COLOR, PLAYER_2_COLOR}, components::{WallBundle, WallLocation, Player1, Player2}};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { player_1_score: 0, player_2_score: 0})
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_event::<CollisionEvent>()
        // Configure how frequently our gameplay systems are run
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_systems(Startup, setup)
        // Add our gameplay simulation systems to the fixed timestep schedule
        .add_systems(
            FixedUpdate,
            (
                check_for_collisions,
                apply_velocity.before(check_for_collisions),
                move_player_1
                    .before(check_for_collisions)
                    .after(apply_velocity),
                move_player_2
                    .before(check_for_collisions)
                    .after(apply_velocity),
                play_collision_sound.after(check_for_collisions),
            ),
        )
        .add_systems(Update, (update_scoreboard, bevy::window::close_on_esc))
        .run();
}

// Add the game's entities to our world
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Sound
    let ball_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    commands.insert_resource(CollisionSound(ball_collision_sound));

    // Paddle
    let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;

    let player_1_x = LEFT_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;
    let player_2_x = RIGHT_WALL - GAP_BETWEEN_PADDLE_AND_FLOOR;

    // Player 1
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(player_1_x, paddle_y, 0.0),
                scale: PLAYER_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PLAYER_1_COLOR,
                ..default()
            },
            ..default()
        },
        Player1,
        Collider,
    ));

    // Player 2
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(player_2_x, paddle_y, 0.0),
                scale: PLAYER_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PLAYER_2_COLOR,
                ..default()
            },
            ..default()
        },
        Player2,
        Collider,
    ));

    // Ball
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(BALL_STARTING_POSITION).with_scale(BALL_SIZE),
            ..default()
        },
        Ball,
        Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
    ));

    // Scoreboard
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }), TextSection::new(
                " - ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            })
        ])
        .with_style(Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            border: UiRect {
                left: Val::Px(10.0),
                right: Val::Px(20.0),
                top: Val::Px(30.0),
                bottom: Val::Px(40.0),
            },
            
            position_type: PositionType::Absolute,
            top: Val::Px(30.0),
            left: Val::Percent(45.0),
            ..default()
        }),
    );

    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));

}

fn move_player_1(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player1>>,
    time_step: Res<FixedTime>,
) {
    let mut player_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::W) {
        direction += 1.0;
    }
    

    if keyboard_input.pressed(KeyCode::S) {
        direction -= 1.0;
    }

    // Calculate the new horizontal paddle position based on player input
    let new_paddle_position =
        player_transform.translation.y + direction * PADDLE_SPEED * time_step.period.as_secs_f32();

    // Update the paddle position,
    // making sure it doesn't cause the paddle to leave the arena
    let top_bound = TOP_WALL - WALL_THICKNESS * 2.0 - PADDLE_SIZE.y * 2.0 - PADDLE_PADDING;
    let bottom_bound = BOTTOM_WALL + WALL_THICKNESS * 2.0 + PADDLE_SIZE.y * 2.0 + PADDLE_PADDING;

    player_transform.translation.y = new_paddle_position
        .clamp(bottom_bound, top_bound);
}


fn move_player_2(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player2>>,
    time_step: Res<FixedTime>,
) {
    let mut player_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Up) {
        direction += 1.0;
    }
    

    if keyboard_input.pressed(KeyCode::Down) {
        direction -= 1.0;
    }

    // Calculate the new horizontal paddle position based on player input
    let new_paddle_position =
        player_transform.translation.y + direction * PADDLE_SPEED * time_step.period.as_secs_f32();

    // Update the paddle position,
    // making sure it doesn't cause the paddle to leave the arena
    let top_bound = TOP_WALL - WALL_THICKNESS * 2.0 - PADDLE_SIZE.y * 2.0 - PADDLE_PADDING;
    let bottom_bound = BOTTOM_WALL + WALL_THICKNESS * 2.0 + PADDLE_SIZE.y * 2.0 + PADDLE_PADDING;

    player_transform.translation.y = new_paddle_position
        .clamp(bottom_bound, top_bound);
}



fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
        transform.translation.y += velocity.y * time_step.period.as_secs_f32();
    }
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.player_1_score.to_string();
    text.sections[3].value = scoreboard.player_2_score.to_string();
}

fn check_for_collisions(
    // mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    // check collision with walls
    for (collider_entity, transform) in &collider_query {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate(),
        );

        // println!("{:#?}", collider_entity.);
        if let Some(collision) = collision {
            // Sends a collision event so that other systems can react to the collision
            collision_events.send_default();

            let entity_index = collider_entity.index();

            // Collides with left wall
            if entity_index ==  6 {
                scoreboard.player_2_score += 1;
            }

            // Collides with right wall
            if entity_index ==  7 {
                scoreboard.player_1_score += 1;
            }

            // // Bricks should be despawned and increment the scoreboard on collision
            // if maybe_brick.is_some() {
            //     scoreboard.score += 1;
            //     commands.entity(collider_entity).despawn();
            // }

            // reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            // only reflect if the ball's velocity is going in the opposite direction of the
            // collision
            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                Collision::Inside => { /* do nothing */ }
            }

            let slight_rotation: f32 =  *vec![-1.0, 1.0, 2.1, 1.5, -1.75, 3.15, -2.25].choose(&mut rand::thread_rng()).unwrap();

            // reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                ball_velocity.x = -ball_velocity.x + slight_rotation;
            }

            // reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                ball_velocity.y = -ball_velocity.y + slight_rotation;
            }
        }
    }
}

fn play_collision_sound(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    sound: Res<CollisionSound>,
) {
    // Play a sound once per frame if a collision occurred.
    if !collision_events.is_empty() {
        // This prevents events staying active on the next frame.
        collision_events.clear();
        commands.spawn(AudioBundle {
            source: sound.0.clone(),
            // auto-despawn the entity when playback finishes
            settings: PlaybackSettings::DESPAWN,
        });
    }
}