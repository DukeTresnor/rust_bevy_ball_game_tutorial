// enemy systems
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;


use crate::game::enemy::components::*;
use crate::game::enemy::resources::*;
use crate::game::enemy::{NUMBER_OF_ENEMIES, ENEMY_SIZE, ENEMY_SPEED};



pub fn spawn_enemies(
    mut commands:  Commands,
    window_query:  Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x_pos = random::<f32>() * window.width();
        let random_y_pos = random::<f32>() * window.height();
        commands.spawn( (
            SpriteBundle {
                transform: Transform::from_xyz(random_x_pos, random_y_pos, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()),
            },
        ));
    }

}

pub fn despawn_enemies(
    mut commands:  Commands,
    //window_query:  Query<&Window, With<PrimaryWindow>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}

// transform and enemy directions
// enemy_query needs two arguments in its Query, because there are multiple enemies,
//   as opposed to the player which there should be only one?
pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
) {
    // For each transform enemy pair in the enemy_query, which is populated with
    //   (transform, enemy) tuples (I think)...

    for (mut transform, enemy) in enemy_query.iter_mut() {
        // Random direction add-ons?
        //let random_x_direction = random::<f32>();
        //let random_y_direction = random::<f32>();
        let direction =  Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        //direction += Vec3::new(random_x_direction, random_y_direction, 0.0);

        /*
        // Normalize the direction
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        */

        // Apply enemy direction at ENEMY_SPEED to the current enemy's transform
        //   Gives smooth, frame-rate independant movement
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}


pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query:  Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let window =  window_query.get_single().unwrap();
        
    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_pos_min = 0.0 + half_enemy_size;
    let x_pos_max = window.width() - half_enemy_size;
    let y_pos_min = 0.0 + half_enemy_size;
    let y_pos_max = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;
        let translation = transform.translation;
        if translation.x < x_pos_min || translation.x > x_pos_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y < y_pos_min || translation.y > y_pos_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }        

        if direction_changed {
            let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");
            let sound_effect = if random::<f32>() > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };
            audio.play(sound_effect);
        }

    }

}


pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query:  Query<&Window, With<PrimaryWindow>>,  
) {
    let window =  window_query.get_single().unwrap();
    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_pos_min = 0.0 + half_enemy_size;
    let x_pos_max = window.width() - half_enemy_size;
    let y_pos_min = 0.0 + half_enemy_size;
    let y_pos_max = window.height() - half_enemy_size;

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

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

        transform.translation = translation;
    }


}


pub fn tick_enemy_spawn_timer(
    time: Res<Time>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>

) {
    enemy_spawn_timer.timer.tick(time.delta());
}


pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>

) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x_pos = random::<f32>() * window.width();
        let random_y_pos = random::<f32>() * window.height();
        commands.spawn( (
            SpriteBundle {
                transform: Transform::from_xyz(random_x_pos, random_y_pos, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()),
            },
        ));
    }
}