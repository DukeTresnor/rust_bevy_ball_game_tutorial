// score mod.rs

use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::*;
use systems::*;
use crate::AppState;
use crate::game::SimulationState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin{ 
    fn build(&self, app: &mut App) {
        app
            //.init_resource::<GatheredStarCount>()
            .init_resource::<HighScores>()
        //    .add_system(update_score)
        //    .add_system(update_high_scores)
        //    .add_system(high_scores_updated);
            .add_system(insert_score.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    update_score,
                    update_high_scores,
                    high_scores_updated,
                )
           
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(remove_score.in_schedule(OnExit(AppState::Game)));
    }
}