// player mod.rs
use bevy::prelude::*;


pub mod components;
mod systems;

use systems::*;
use crate::AppState;
use crate::game::SimulationState;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_startup_system(spawn_player)
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            //.add_system(player_movement)
            //.add_system(confine_player_movement)
            //.add_system(enemy_hit_player)
            //.add_system(player_hit_star);
            .add_systems(
                (
                    player_movement,
                    confine_player_movement,
                    enemy_hit_player,
                    player_hit_star,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running))
            )
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));

    }
}