use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::defense::resources::*;
use crate::defense::components::*;

pub fn tower_selection(mut contexts: EguiContexts, asset_server: Res<AssetServer>, mut defense_selection: ResMut<DefenseSelection>) {
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
                if ui.add(egui::Button::image(egui::load::SizedTexture::new(canon_tower_image, [320.0, 320.0]))).clicked() {
                    defense_selection.selected = Weapon::Cannon;
                    info!("Tower 1 selected");
                }

                if ui.add(egui::Button::image(egui::load::SizedTexture::new(ballista_tower_image, [320.0, 320.0]))).clicked() {
                    defense_selection.selected = Weapon::Ballista;
                    info!("Tower 2 selected");
                }

                if ui.add(egui::Button::image(egui::load::SizedTexture::new(archer_tower_image, [320.0, 320.0]))).clicked() {
                    defense_selection.selected = Weapon::Archer;
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
