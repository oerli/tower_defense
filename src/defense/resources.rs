use bevy::prelude::*;

use crate::defense::components::*;

#[derive(Resource)]
pub struct DefenseSelection {
    pub selected: Weapon,
}

impl Default for DefenseSelection {
    fn default() -> Self {
        DefenseSelection {
            selected: Weapon::Cannon,
        }
    }
}
