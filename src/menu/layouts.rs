use bevy::prelude::*;
use bevy_egui::{egui, egui::Align2, egui::Label, egui::RichText, egui::Vec2, EguiContexts};

use crate::defense::components::*;
use crate::defense::resources::*;
use crate::player::resources::*;
use crate::GameState;
use crate::HoverHandler;

pub fn tower_selection(
    mut contexts: EguiContexts,
    asset_server: Res<AssetServer>,
    mut defense_selection: ResMut<DefenseSelection>,
    mut hover_handler: ResMut<HoverHandler>,
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let canon_tower_image = contexts.add_image(asset_server.load("images/canon_tower.png"));
    let ballista_tower_image = contexts.add_image(asset_server.load("images/ballista_tower.png"));
    let archer_tower_image = contexts.add_image(asset_server.load("images/archer_tower.png"));

    egui::Window::new("Select Tower")
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .anchor(Align2::CENTER_CENTER, Vec2::new(0.0, 0.0))
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

                    // create cannon tower for hover events
                    if hover_handler.entity.is_some() {
                        commands
                            .entity(hover_handler.entity.unwrap())
                            .despawn_recursive();
                    }
                    hover_handler.entity = Some(
                        commands
                            .spawn(SceneBundle {
                                scene: asset_server.load("models/cannon_tower.glb#Scene0"),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent.spawn((PbrBundle {
                                    mesh: asset_server.add(Mesh::from(shape::Torus {
                                        radius: 2.96,
                                        ring_radius: 0.02,
                                        ..Default::default()
                                    })),
                                    material: asset_server.add(StandardMaterial {
                                        base_color: Color::rgb(0.8, 0.2, 0.2),
                                        ..Default::default()
                                    }),
                                    transform: Transform::from_xyz(0.0, 0.3, 0.0),
                                    ..Default::default()
                                },));
                            })
                            .id(),
                    );

                    next_game_state.set(GameState::Paused);
                }

                if ui
                    .add(egui::Button::image(egui::load::SizedTexture::new(
                        ballista_tower_image,
                        [320.0, 320.0],
                    )))
                    .clicked()
                {
                    defense_selection.selected = Weapon::Ballista;

                    // create ballista tower for hover events
                    if hover_handler.entity.is_some() {
                        commands
                            .entity(hover_handler.entity.unwrap())
                            .despawn_recursive();
                    }
                    hover_handler.entity = Some(
                        commands
                            .spawn(SceneBundle {
                                scene: asset_server.load("models/ballista_tower.glb#Scene0"),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent.spawn((PbrBundle {
                                    mesh: asset_server.add(Mesh::from(shape::Torus {
                                        radius: 2.96,
                                        ring_radius: 0.02,
                                        ..Default::default()
                                    })),
                                    material: asset_server.add(StandardMaterial {
                                        base_color: Color::rgb(0.8, 0.2, 0.2),
                                        ..Default::default()
                                    }),
                                    transform: Transform::from_xyz(0.0, 0.3, 0.0),
                                    ..Default::default()
                                },));
                            })
                            .id(),
                    );

                    next_game_state.set(GameState::Paused);
                }

                if ui
                    .add(egui::Button::image(egui::load::SizedTexture::new(
                        archer_tower_image,
                        [320.0, 320.0],
                    )))
                    .clicked()
                {
                    defense_selection.selected = Weapon::Archer;

                    // create archer tower for hover events
                    if hover_handler.entity.is_some() {
                        commands
                            .entity(hover_handler.entity.unwrap())
                            .despawn_recursive();
                    }
                    hover_handler.entity = Some(
                        commands
                            .spawn(SceneBundle {
                                scene: asset_server.load("models/archer_tower.glb#Scene0"),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent.spawn((PbrBundle {
                                    mesh: asset_server.add(Mesh::from(shape::Torus {
                                        radius: 1.96,
                                        ring_radius: 0.02,
                                        ..Default::default()
                                    })),
                                    material: asset_server.add(StandardMaterial {
                                        base_color: Color::rgb(0.8, 0.2, 0.2),
                                        ..Default::default()
                                    }),
                                    transform: Transform::from_xyz(0.0, 0.3, 0.0),
                                    ..Default::default()
                                },));
                            })
                            .id(),
                    );

                    next_game_state.set(GameState::Paused);
                }

                ui.end_row();

                ui.label("Cannon Tower\nCredits: 1\nRange: 3, Frequency: 1s, Damage: 0.5");
                ui.label("Ballista Tower\nCredits: 1\nRange: 3, Frequency: 0.5s, Damage: 0.3");
                ui.label("Archer Tower\nCredits: 1\nRange: 3, Frequency: 0.2s, Damage: 0.1");

                ui.end_row();
            });
        });
}

pub fn high_scores(mut contexts: EguiContexts, player: Res<Player>) {
    egui::Window::new("High Scores")
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .anchor(Align2::CENTER_CENTER, Vec2::new(0.0, 0.0))
        .show(contexts.ctx_mut(), |ui| {
            ui.add_sized(
                Vec2::new(ui.available_width(), 0.0),
                Label::new(RichText::new("Game Over!").heading().strong()),
            );
            ui.end_row();

            egui::Grid::new("players")
                .striped(true)
                .num_columns(5)
                .spacing([40.0, 4.0])
                .show(ui, |ui| {
                    ui.label(RichText::new("Name").heading());
                    ui.label(RichText::new("Level").heading());
                    ui.label(RichText::new("Lives").heading());
                    ui.label(RichText::new("Score").heading());
                    ui.label(RichText::new("Credits").heading());
                    ui.end_row();

                    ui.label(format!("{}", player.name));
                    ui.label(format!("{}", player.level));
                    ui.label(format!("{}", player.lives));
                    ui.label(format!("{}", player.score));
                    ui.label(format!("{}", player.credits));

                    ui.end_row();
                });
        });
}

pub fn show_controls(
    mut contexts: EguiContexts,
    mut next_game_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    game_state: Res<State<GameState>>,
    player: Res<Player>,
) {
    let play_image = contexts.add_image(asset_server.load("images/play.png"));
    let pause_image = contexts.add_image(asset_server.load("images/pause.png"));

    let (play_enabled, pause_enabled) = if *game_state.get() == GameState::Paused {
        (true, false)
    } else if *game_state.get() == GameState::Playing {
        (false, true)
    } else {
        (false, false)
    };

    egui::Window::new("Pause Menu")
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .anchor(Align2::CENTER_TOP, Vec2::new(0.0, 0.0))
        .show(contexts.ctx_mut(), |ui| {
            egui::Grid::new("pause").min_col_width(48.0).show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add(
                        egui::Label::new(
                            RichText::new(format!("Lives\n{}", player.lives)).heading(),
                        )
                        .wrap(false),
                    );
                });
                ui.vertical_centered(|ui| {
                    ui.add(
                        egui::Label::new(
                            RichText::new(format!("Level\n{}", player.level)).heading(),
                        )
                        .wrap(false),
                    );
                });

                if ui
                    .add_enabled(
                        play_enabled,
                        egui::Button::image(egui::load::SizedTexture::new(
                            play_image,
                            [48.0, 48.0],
                        )),
                    )
                    .clicked()
                {
                    next_game_state.set(GameState::Playing);
                };
                if ui
                    .add_enabled(
                        pause_enabled,
                        egui::Button::image(egui::load::SizedTexture::new(
                            pause_image,
                            [48.0, 48.0],
                        )),
                    )
                    .clicked()
                {
                    next_game_state.set(GameState::Paused);
                };

                ui.vertical_centered(|ui| {
                    ui.add(
                        egui::Label::new(
                            RichText::new(format!("Credits\n{}", player.credits)).heading(),
                        )
                        .wrap(false),
                    );
                });
                ui.vertical_centered(|ui| {
                    ui.add(
                        egui::Label::new(
                            RichText::new(format!("Score\n{}", player.score)).heading(),
                        )
                        .wrap(false),
                    );
                });
                ui.end_row();
            });
        });
}
