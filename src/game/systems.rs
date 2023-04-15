// game/ systems

use bevy::prelude::*;

use crate::game::SimulationState;


pub fn pause_simulation(
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    next_simulation_state.set(SimulationState::Paused);
}

pub fn resume_simulation(
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    next_simulation_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    // inputs
    //mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    // is just pressed() for buttons instead of things meant to be
    //   held?
    // simulation_state.0 represents the current state
    if keyboard_input.just_pressed(KeyCode::Space) {
        if simulation_state.0 == SimulationState::Running {
            //commands.insert_resource(NextState(Some(SimulationState::Paused)));
            next_simulation_state.set(SimulationState::Paused);
            println!("Simulation Paused");
        }
        if simulation_state.0 == SimulationState::Paused {
            //commands.insert_resource(NextState(Some(SimulationState::Running)));
            next_simulation_state.set(SimulationState::Running);
            println!("Simulation Running");
        }
    }
}