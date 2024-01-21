use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Event)]
pub struct BuildEvent {
    pub button: PointerButton,
    pub entity: Entity,
}

impl From<ListenerInput<Pointer<Click>>> for BuildEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        BuildEvent {
            button: event.event.button,
            entity: event.target,
        }
    }
}
