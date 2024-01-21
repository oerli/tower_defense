use bevy::prelude::*;
use bevy::transform::commands;
use bevy_egui::{egui, EguiContexts};

use crate::defense::components::*;
use crate::defense::resources::*;
use crate::HoverHandler;

pub fn tower_selection(
    mut contexts: EguiContexts,
    asset_server: Res<AssetServer>,
    mut defense_selection: ResMut<DefenseSelection>,
    mut hover_handler: ResMut<HoverHandler>,
    mut commands: Commands,
) {
    let canon_tower_image = contexts.add_image(asset_server.load("images/canon_tower.png"));
    let ballista_tower_image = contexts.add_image(asset_server.load("images/ballista_tower.png"));
    let archer_tower_image = contexts.add_image(asset_server.load("images/archer_tower.png"));

    egui::Window::new("Select Tower")
        // .vscroll(false)
        // .default_width(160.0)
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .default_pos([140.0, 140.0])
        // .pivot(egui::Align2::CENTER_CENTER)
        // .open(&mut ui_state.is_window_open)
        .show(contexts.ctx_mut(), |ui| {
            egui::Grid::new("selection").striped(true).show(ui, |ui| {
                if ui
                    .add(egui::Button::image(egui::load::SizedTexture::new(
                        canon_tower_image,
                        [320.0, 320.0],
                    )))
                    .clicked()
                {
                    defense_selection.selected = Weapon::Cannon;

                    // create tower for hover events
                    if hover_handler.entity.is_some() {
                        commands.entity(hover_handler.entity.unwrap()).despawn_recursive();
                    }
                    hover_handler.entity = Some(
                        commands
                            .spawn(SceneBundle {
                                scene: asset_server.load("models/cannon_tower.glb#Scene0"),
                                ..Default::default()
                            })
                            .id(),
                    );

                    info!("Tower 1 selected");
                }

                if ui
                    .add(egui::Button::image(egui::load::SizedTexture::new(
                        ballista_tower_image,
                        [320.0, 320.0],
                    )))
                    .clicked()
                {
                    defense_selection.selected = Weapon::Ballista;

                    // create tower for hover events
                    if hover_handler.entity.is_some() {
                        commands.entity(hover_handler.entity.unwrap()).despawn_recursive();
                    }
                    hover_handler.entity = Some(
                        commands
                            .spawn(SceneBundle {
                                scene: asset_server.load("models/ballista_tower.glb#Scene0"),
                                ..Default::default()
                            })
                            .id(),
                    );

                    info!("Tower 2 selected");
                }

                if ui
                    .add(egui::Button::image(egui::load::SizedTexture::new(
                        archer_tower_image,
                        [320.0, 320.0],
                    )))
                    .clicked()
                {
                    defense_selection.selected = Weapon::Archer;

                    // create tower for hover events
                    if hover_handler.entity.is_some() {
                        commands.entity(hover_handler.entity.unwrap()).despawn_recursive();
                    }
                    hover_handler.entity = Some(
                        commands
                            .spawn(SceneBundle {
                                scene: asset_server.load("models/archer_tower.glb#Scene0"),
                                ..Default::default()
                            })
                            .id(),
                    );

                    info!("Tower 3 selected");
                }

                ui.end_row();

                ui.label("Tower 1 description");
                ui.label("Tower 2 description");
                ui.label("Tower 3 description");

                ui.end_row();
            });
        });
}
