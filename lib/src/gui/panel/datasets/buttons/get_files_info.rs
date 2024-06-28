use std::str::FromStr;

use crate::global::{self, get_state};
use crate::types::{collection::Collection, dataset::Dataset};

pub fn get_files_info(ui: &mut egui::Ui, collection: Collection, dataset: Dataset) {
    btn(ui, "filelist.txt", dataset.clone(), collection.clone());
    btn(ui, "checksums.md5", dataset.clone(), collection.clone());
    // btn_or_progress(ui, "filelist.txt", dataset.cloe(), collection.clone());
    // btn_or_progress(ui, "checksums.md5", dataset.clone(), collection.clone());
}

fn btn(ui: &mut egui::Ui, filename: &str, dataset: Dataset, collection: Collection) {
    let state = get_state();
    let path = state
        .working_directory
        .join("data")
        .join(collection.alias.clone())
        .join(dataset.alias.clone());

    let key = format!(
        "{}_{}_{}",
        collection.alias.clone(),
        dataset.alias.clone(),
        filename
    );

    let filename = String::from_str(filename).unwrap();
    if !path.join(filename.clone()).exists()
        && ui.button(format!("Get {}", filename.clone())).clicked()
    {
        let psr_arc = global::add_or_get_progress(key);

        let c = collection.clone();
        let d = dataset.clone();
        let sr = psr_arc.clone();
        let f = filename.clone();

        tokio::spawn(async move {
            d.get_crcns_file(c.alias.clone().as_str(), f.as_str(), sr)
                .await;
        });
    }
}

// fn btn_or_progress(ui: &mut egui::Ui, filename: &str, dataset: Dataset, collection: Collection) {
//     let state = get_state();
//     let path = state
//         .working_directory
//         .join("data")
//         .join(collection.alias.clone())
//         .join(dataset.alias.clone());
//
//     let key = format!(
//         "{}_{}_{}",
//         collection.alias.clone(),
//         dataset.alias.clone(),
//         filename
//     );
//
//     if !path.join(filename).exists()
//         && !state.progress_done.lock().unwrap().contains(&key)
//         && ui.button(format!("Get {}", filename)).clicked()
//     {
//         let psr_arc = global::add_or_get_progress(key);
//
//         let c = collection.clone();
//         let d = dataset.clone();
//         let sr = psr_arc.clone();
//
//         tokio::spawn(async move {
//             d.get_crcns_file(c.alias.clone().as_str(), "filelist.txt", sr)
//                 .await;
//         });
//     } else {
//         let p = state.progress.lock().unwrap();
//         let p = p.get(&key);
//         let p = p.unwrap_or(&0.0);
//         ui.add(egui::widgets::ProgressBar::new(*p));
//     }
// }
