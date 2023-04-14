// score systems

use bevy::prelude::*;
//use bevy::transform::commands;


use crate::events::*;
use crate::game::score::resources::*;

pub fn insert_score(
    mut commands: Commands,
) {
    commands.insert_resource(GatheredStarCount::default());
}

pub fn remove_score(
    mut commands: Commands,
) {
    commands.remove_resource::<GatheredStarCount>();
}

pub fn update_score(
    star_resource: Res<GatheredStarCount>,
) {
    if star_resource.is_changed() {
        println!("Collected Star. Star count changed to: {}", star_resource.count.to_string());
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    // Iterate over the events of type Game Over
    for event in game_over_event_reader.iter() {
        high_scores.scores.push(("Player".to_string(), event.score));
    }
}


pub fn high_scores_updated(
    high_scores: Res<HighScores>,
) {
    // You can call is_changed() on a resource to check if it's changed since the last
    //   frame
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
    }
}