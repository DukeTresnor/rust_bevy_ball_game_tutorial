// enemy components

use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,   // Enemy can keep track of where it's moving
}