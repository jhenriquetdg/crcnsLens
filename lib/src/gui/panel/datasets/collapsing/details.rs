use crate::global;
use crate::gui::panel::datasets::buttons;

use crate::types::{collection::Collection, dataset::Dataset};

pub fn details(ui: &mut egui::Ui, collection: Collection, dataset: Dataset) {
    ui.collapsing(dataset.alias.clone(), |ui| {
        let state_dataset = global::get_state_dataset();

        if state_dataset == dataset && ui.button("x").clicked() {
            global::set_state_dataset(Dataset::default());
            buttons::get_files_info(ui, collection.clone(), dataset.clone());
            buttons::set_dataset(ui, collection.clone(), dataset.clone());
            buttons::view_filelist(ui, collection.clone(), dataset.clone());

            let files = global::get_state_dataset_files();

            for file in files {
                ui.label(file.remote_path);
            }
        } else if ui.button("o").clicked() {
            global::set_state_dataset(dataset.clone());
        }

        // ui.label(dataset.description.clone());
        // ui.label(dataset.last_modified.clone());
        // ui.label(dataset.description.clone());
        // ui.label(dataset.url.clone());
    });
}
