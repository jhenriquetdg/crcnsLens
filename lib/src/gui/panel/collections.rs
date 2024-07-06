pub mod buttons;
pub mod collapsing;

use crate::global;
use crate::gui::traits;
use crate::types::Collection;

pub struct CollectionPanel {
    pub is_open: bool,
    pub collection: Collection,
}

impl Default for CollectionPanel {
    fn default() -> Self {
        Self {
            is_open: true,
            collection: Collection::default(),
        }
    }
}

impl traits::View for CollectionPanel {
    fn ui(&mut self, _ui: &mut egui::Ui) {}

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("collection_panel")
            .resizable(true)
            .show_animated(ctx, self.is_open, |ui| {
                let state = global::get_state();

                ui.vertical_centered(|ui| {
                    ui.heading("💻 Collections");
                });

                ui.separator();

                let current_collection = global::get_state_collection();
                let layout = egui::Layout::top_down(egui::Align::RIGHT);
                ui.allocate_ui_with_layout(ui.available_size(), layout, |ui| {
                    egui::ScrollArea::both().show(ui, |ui| {
                        if current_collection.clone() == Collection::default() {
                            // Add a lot of widgets here.
                            let collection_arc = state.collections.clone();
                            let collection_result = collection_arc.lock();
                            match collection_result {
                                Ok(collection_mutex) => {
                                    for (c, collection) in
                                        collection_mutex.clone().into_iter().enumerate()
                                    {
                                        ui.push_id(
                                            format!(
                                                "ch_collection_{}_{}",
                                                c,
                                                collection.alias.clone()
                                            ),
                                            |ui| {
                                                collapsing::details(ui, collection);
                                            },
                                        );
                                    }
                                }
                                Err(_e) => (),
                            };
                        } else {
                            let collection = current_collection.clone();

                            ui.push_id(
                                format!("ch_collection_selected_{}", collection.alias.clone()),
                                |ui| {
                                    collapsing::details(ui, collection);
                                },
                            );
                        }
                    });

                    buttons::load_persisted(ui);
                });
            });
    }
}

impl traits::Display for CollectionPanel {
    fn name(&self) -> &'static str {
        ""
    }
    fn show(&mut self, _ctx: &egui::Context, _open: &mut bool) {}
    fn is_enabled(&self, _ctx: &egui::Context) -> bool {
        true
    }
}
