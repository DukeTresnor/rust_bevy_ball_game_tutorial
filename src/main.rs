// main file -- run plugins


pub mod events;
mod systems;
mod game;
mod main_menu;

use bevy::prelude::*;

//use events::*;
use systems::*;
use game::GamePlugin;
use main_menu::MainMenuPlugin;



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Adding States
        .add_state::<AppState>()
        // My Plugins
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        // Startup Systems
        .add_startup_system(spawn_camera)
        // Systems
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}

// Determines if we are on the MainMenu, Game, or GameOver state
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}