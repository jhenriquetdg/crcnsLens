use crate::global;

use crate::types::collection::Collection;
use crate::types::dataset::Dataset;

pub fn load_persisted(ui: &mut egui::Ui) {
    if ui.button("Load persisted").clicked() {
        let state = global::get_state();

        let c = state.collections.clone();
        let data_directory_path = state.working_directory.clone().join("data");

        for col in std::fs::read_dir(data_directory_path)
            .unwrap()
            .filter_map(|f| f.ok())
        {
            println!("*{:?}", col.path());
            let collection = Collection::from_filepath(col.path());

            let d = collection.datasets.clone();

            for (ds, ds_file) in std::fs::read_dir(col.path())
                .unwrap()
                .filter_map(|f| f.ok())
                .filter(|f| f.path().is_dir())
                .enumerate()
            {
                println!("\t{ds}: {:?}", ds_file.path());
                let dataset = Dataset::from_filepath(ds_file.path());
                d.lock().unwrap().push(dataset);
            }
            c.lock().unwrap().push(collection);
        }
    }
}
