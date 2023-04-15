// main systems

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::events::*;
use crate::AppState;
use crate::game::SimulationState;

pub fn spawn_camera(
    mut commands: Commands,
    window_query:  Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(  
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
    );

}

//  Menu --> Game transitions  //

// these want to take in AppState because AppState is that state enum
//   that exists at the src level of this projec (not the T)
pub fn transition_to_game_state(
    //mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    // If you press G
    if keyboard_input.just_pressed(KeyCode::G) {
        // and if you're not in the Game state (defined in AppState)
        if app_state.0 != AppState::Game {
            // then transition to the Game state
            next_app_state.set(AppState::Game);
            //commands.insert_resource(NextState(Some(AppState::Game)));
            println!("Entered AppState::Game");
        }
    }
}

pub fn transition_to_main_menu_state(
    // input
    //mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if app_state.0 != AppState::MainMenu {
            //commands.insert_resource(NextState(Some(AppState::MainMenu)));
            //commands.insert_resource(NextState(Some(SimulationState::Paused)));
            next_app_state.set(AppState::MainMenu);
            next_simulation_state.set(SimulationState::Paused);
            println!("Entered AppState::MainMenu");
        }
    }
}
//  Menu --> Game transitions  //


pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    // close when esc is pressed
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

// this needs to accept and hndle the game over event that's being sent out by
//   the enemy_hit_player() system
pub fn handle_game_over(
    //mut commands: Commands,
    // game_over_event_reader is a receiver of GameOver events (and not other events)
    mut game_over_event_reader: EventReader<GameOver>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for event in game_over_event_reader.iter() {
        println!("Your final score is: {}", event.score.to_string());
        //commands.insert_resource(NextState(Some(AppState::GameOver)));
        next_app_state.set(AppState::GameOver);
    }
}