use bevy::prelude::*;

#[derive(Reflect, Resource)]
pub struct Player {
    pub name: String,
    pub lives: i32,
    pub score: i32,
    pub level: i32,
    pub credits: i32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            name: "Player".to_string(),
            lives: 10,
            score: 0,
            level: 1,
            credits: 1,
        }
    }
}
