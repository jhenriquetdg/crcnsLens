use crate::{global::set_state_dataset, types::Dataset};

pub fn set_dataset(ui: &mut egui::Ui, dataset: Dataset) {
    if ui.button("Set dataset").clicked() {
        set_state_dataset(dataset);
    }
}

