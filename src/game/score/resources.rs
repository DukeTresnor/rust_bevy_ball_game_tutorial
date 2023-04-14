// score resources

use bevy::prelude::*;

#[derive(Resource)]
pub struct GatheredStarCount {
    pub count: u32,
}

// Default is the constructor for the GatheredStarCount resource
impl Default for GatheredStarCount {
    fn default() -> GatheredStarCount {
        GatheredStarCount { count: 0 }
    }
}

#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScores {
    fn default() -> HighScores {
        HighScores {
            scores: Vec::new()
        }
    }
}