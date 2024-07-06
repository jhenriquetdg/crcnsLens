use crate::files::handlers::load_or_download_filelist;
use crate::types::{Collection, Dataset};

pub fn view_filelist(ui: &mut egui::Ui, collection: Collection, dataset: Dataset) {
    if ui.button("View files").clicked() {
        tokio::spawn(async move {
            load_or_download_filelist(collection, dataset).await;
        });
    }
}
