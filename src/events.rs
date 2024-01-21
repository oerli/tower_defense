use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::resources::*;

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

#[derive(Event)]
pub struct OverEvent {
    pub entity: Entity,
}

impl From<ListenerInput<Pointer<Over>>> for OverEvent {
    fn from(event: ListenerInput<Pointer<Over>>) -> Self {
        OverEvent {
            entity: event.target,
        }
    }
}

pub fn hover_event(
    mut hover_events: EventReader<OverEvent>,
    hover_handler: ResMut<HoverHandler>,
    transform_query: Query<&GlobalTransform>,
    mut transform: Query<&mut Transform>,
    mut visibility_query: Query<&mut Visibility>,
) {
    for event in hover_events.read() {
        match hover_handler.entity {
            Some(entity) => {
                transform.get_mut(entity).unwrap().translation =
                    transform_query.get(event.entity).unwrap().translation()
                        + Vec3::new(0.0, 0.1, 0.0);
                *visibility_query.get_mut(entity).unwrap() = Visibility::Visible;
            }
            None => {}
        }
    }
}

// #[derive(Event)]
// pub struct OutEvent {
//     pub entity: Entity,
// }

// impl From<ListenerInput<Pointer<Out>>> for OutEvent {
//     fn from(event: ListenerInput<Pointer<Out>>) -> Self {
//         OutEvent {
//             entity: event.target,
//         }
//     }
// }

// pub fn out_event(
//     mut out_events: EventReader<OutEvent>,
//     hover_handler: Res<HoverHandler>,
//     mut visibility_query: Query<&mut Visibility>,
// ) {
//     for _event in out_events.read() {
//         match hover_handler.entity {
//             Some(entity) => {
//                 *visibility_query.get_mut(entity).unwrap() = Visibility::Hidden;
//             }
//             None => {}
//         }
//     }
// }
