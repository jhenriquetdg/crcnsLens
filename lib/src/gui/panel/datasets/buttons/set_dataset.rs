use crate::{
    global::{set_state_collection, set_state_dataset},
    types::{Collection, Dataset},
};

pub fn set_dataset(ui: &mut egui::Ui, collection: Collection, dataset: Dataset) {
    if ui.button("Set dataset").clicked() {
        set_state_collection(collection);
        set_state_dataset(dataset);
    }
}
