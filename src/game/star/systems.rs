// star systems


use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::game::star::resources::*;
use crate::game::star::components::Star;

use crate::game::star::{NUMBER_OF_STARS};

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_STARS {
        let random_x_pos = random::<f32>() * window.width();
        let random_y_pos = random::<f32>() * window.height();
        commands.spawn( (
            SpriteBundle {
                transform: Transform::from_xyz(random_x_pos, random_y_pos, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn despawn_stars(
    mut commands: Commands,
    star_query: Query<Entity, With<Star>>,
) {
    for star_entity in star_query.iter() {
        // use the entity method which is a member methodf of Commands, then run despawn
        //   on that entity
        commands.entity(star_entity).despawn()
    }
}

/*

pub fn despawn_enemies(
    mut commands:  Commands,
    //window_query:  Query<&Window, With<PrimaryWindow>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}

 */



pub fn tick_star_spawn_timer(
    time: Res<Time>,
    mut star_spawn_timer: ResMut<StarSpawnTimer>

) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>

) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x_pos = random::<f32>() * window.width();
        let random_y_pos = random::<f32>() * window.height();
        commands.spawn( (
            SpriteBundle {
                transform: Transform::from_xyz(random_x_pos, random_y_pos, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}