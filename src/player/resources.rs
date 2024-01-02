use bevy::prelude::*;

#[derive(Reflect, Resource)]
pub struct Player {
    pub name: String,
    pub score: i32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            name: "Player".to_string(),
            score: 0,
        }
    }
}