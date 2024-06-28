use crate::{
    global,
    types::{Collection, Dataset, File},
};
use std::{
    io::BufRead,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc;

pub async fn load_or_download_filelist(collection: Collection, dataset: Dataset) {
    let state = global::get_state();

    let dir_path = state
        .working_directory
        .join("data")
        .join(collection.alias.clone())
        .join(dataset.alias.clone());

    download_file(
        String::from_str("filelist.txt").unwrap(),
        dir_path.clone(),
        collection.clone(),
        dataset.clone(),
    )
    .await;

    let filelist_filepath = dir_path.join("filelist.txt");
    let filelist_file = std::fs::File::open(filelist_filepath).unwrap();
    let filelist_reader = std::io::BufReader::new(filelist_file);

    let mut remote_paths: Vec<String> = Vec::new();
    let mut remote_sizes: Vec<u64> = Vec::new();

    for line in filelist_reader.lines() {
        let line = line.unwrap();
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }

        let fields: Vec<&str> = line.split_whitespace().collect();

        if fields.clone().len() < 2 {
            continue;
        }

        remote_paths.push(fields[0].to_string());
        remote_sizes.push(fields[1].parse::<u64>().unwrap());
    }

    download_file(
        String::from_str("checksums.md5").unwrap(),
        dir_path.clone(),
        collection.clone(),
        dataset.clone(),
    )
    .await;

    let checksums_filepath = dir_path.join("checksums.md5");
    let checksums_file = std::fs::File::open(checksums_filepath).unwrap();
    let checksums_reader = std::io::BufReader::new(checksums_file);

    let mut remote_md5s: Vec<String> = Vec::new();

    for line in checksums_reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }

        let fields: Vec<&str> = line.split_whitespace().collect();

        if fields.clone().len() < 2 {
            continue;
        }

        remote_md5s.push(fields[0].to_string());
    }

    let mut files: Vec<File> = Vec::new();

    for f in 0..remote_md5s.len() {
        let local_path = dir_path
            .join(remote_paths[f].clone())
            .to_str()
            .unwrap()
            .to_string();

        files.push(File {
            remote_md5: remote_md5s[f].clone(),
            remote_path: remote_paths[f].clone(),
            remote_size: remote_sizes[f],
            local_path,
            local_size: 0,
            local_md5: "".to_string(),
            extension: "".to_string(),
        });
    }

    global::set_state_dataset_files(files);
}

async fn download_file(
    filename: String,
    dir_path: PathBuf,
    collection: Collection,
    dataset: Dataset,
) {
    let filelist_path = dir_path.join(filename.clone());

    if !filelist_path.exists() {
        let _ = tokio::spawn(async move {
            dataset
                .get_crcns_file(
                    collection.alias.clone().as_str(),
                    filename.as_str(),
                    Arc::new(Mutex::new(mpsc::unbounded_channel())),
                )
                .await;
        })
        .await;
    }
}
