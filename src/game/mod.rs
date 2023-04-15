mod player;
pub mod enemy;
pub mod star;
pub mod score;
mod systems;

use bevy::prelude::*;

use enemy::EnemyPlugin;
use star::StarPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use systems::*;

use crate::events::GameOver;
use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // States -- this is on top of the AppState, since that goes with bevy I
            //   think
            .add_state::<SimulationState>()
            // Events
            .add_event::<GameOver>()
            // OnEnter Systems
            .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            // Plugins
            .add_plugin(EnemyPlugin)
            .add_plugin(StarPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ScorePlugin)
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            // When you exit the game state, resume?
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
