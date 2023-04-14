// player systems

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

// use crate::stuff is used when you want to reference code in other modules
//   use crate::enemy::components::*; looks into the enemy module, which is in the src
//   folder; the enemy folder is at the same level as the player folder

use crate::game::enemy::ENEMY_SIZE;
use crate::game::star::STAR_SIZE;

use crate::game::enemy::components::*;
use crate::game::star::components::*;
use crate::game::score::resources::*;
use crate::events::GameOver;

use crate::game::player::components::*;
use crate::game::player::{PLAYER_SPEED, PLAYER_SIZE};



pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn( (
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            //texture: asset_server.load("sprites/lenneth_idle.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn despawn_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn()
    }
}


// Where inputs are processed
pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query:  Query<&mut Transform, With<Player>>,
    window_query:  Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window =  window_query.get_single().unwrap();
        
        let half_player_size = PLAYER_SIZE / 2.0;
        let x_pos_min = 0.0 + half_player_size;
        let x_pos_max = window.width() - half_player_size;
        let y_pos_min = 0.0 + half_player_size;
        let y_pos_max = window.height() - half_player_size;
        //
        let mut translation = player_transform.translation;

        if translation.x < x_pos_min {
            translation.x = x_pos_min;
        } else if translation.x > x_pos_max {
            translation.x = x_pos_max;
        }

        if translation.y < y_pos_min {
            translation.y = y_pos_min;
        } else if translation.y > y_pos_max {
            translation.y = y_pos_max;
        }

        player_transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query:  Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    star_resource: Res<GatheredStarCount>,
) {
    // player_quesry.get_single_mut() returns either a type, or an Error
    //   in this case the type is a tuple that contains the player entity
    //   and the player's transform
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            // distance is set to the distance between the player transform and the 
            //   enemy transform
            let distance = player_transform.translation.distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("Enemy hit player! Game Over!");
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound_effect);
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver {score: star_resource.count});
                // this was covered in the handle_game_over() system 
                //   println!("Score: {}", star_resource.count);
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query:  Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    mut star_resource: ResMut<GatheredStarCount>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform.translation.distance(star_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;
            if distance < player_radius + star_radius {
                star_resource.count += 1;
                println!("Collected Star. Star count is: {}", star_resource.count);
                let sound_effect = asset_server.load("audio/laserLarge_000.ogg");
                audio.play(sound_effect);
                commands.entity(star_entity).despawn();
            }
        }
    }
}