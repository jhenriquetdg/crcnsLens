use crate::global;
use crate::gui::panel::datasets;
use crate::types::collection::Collection;
use crate::types::Dataset;

pub fn details(ui: &mut egui::Ui, collection: Collection) {
    ui.collapsing(collection.descriptor.clone(), |ui| {
        let state_collection = global::get_state_collection();

        if state_collection == collection && ui.button("x").clicked() {
            global::set_state_collection(Collection::default());
        } else if ui.button("o").clicked() {
            global::set_state_collection(collection.clone());
        }

        let state_dataset = global::get_state_dataset();

        if state_dataset == Dataset::default() {
            let dataset_arc = collection.datasets.clone();
            let dataset_result = dataset_arc.lock();
            match dataset_result {
                Ok(dataset_mutex) => {
                    for (d, dataset) in dataset_mutex.clone().into_iter().enumerate() {
                        ui.push_id(
                            format!("ch_dataset_{}_{}", d, dataset.alias.clone()),
                            |ui| {
                                datasets::collapsing::details(
                                    ui,
                                    collection.clone(),
                                    dataset.clone(),
                                );
                            },
                        );
                    }
                }
                Err(e) => {
                    ui.label(format!("Dataset not loaded: {e}").as_str());
                }
            };
        } else {
            let dataset = state_dataset.clone();
            ui.push_id(
                format!("ch_selected_dataset_{}", dataset.alias.clone()),
                |ui| datasets::collapsing::details(ui, collection, dataset),
            );
        };
    });
}
