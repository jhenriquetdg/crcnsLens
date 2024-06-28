use crate::types::{Collection, Dataset};

pub fn view_filelist(ui: &mut egui::Ui, collection: Collection, dataset: Dataset) {
    if ui.button("View files").clicked() {}
}
