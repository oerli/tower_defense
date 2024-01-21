use bevy::prelude::*;

#[derive(Resource)]
pub struct Animations(pub Vec<Handle<AnimationClip>>);

#[derive(Resource)]
pub struct HoverHandler {
    pub entity: Option<Entity>,
}

impl Default for HoverHandler {
    fn default() -> Self {
        HoverHandler {
            entity: None,
        }
    }
}
