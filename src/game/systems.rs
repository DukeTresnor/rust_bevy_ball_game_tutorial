// game/ systems

use bevy::prelude::*;

use crate::game::SimulationState;

pub fn toggle_simulation(
    // inputs
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    // is just pressed() for buttons instead of things meant to be
    //   held?
    // simulation_state.0 represents the current state
    if keyboard_input.just_pressed(KeyCode::Space) {
        if simulation_state.0 == SimulationState::Running {
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
            println!("Simulation Paused");
        }
        if simulation_state.0 == SimulationState::Paused {
            commands.insert_resource(NextState(Some(SimulationState::Running)));
            println!("Simulation Running");
        }
    }
}