use std::sync::{Arc, Mutex};

use crate::gui::app::Lens;
use crate::types::state::SrPair;
use crate::types::State;
use crate::types::{Collection, Dataset, File};

use once_cell::sync::OnceCell;
use polars::lazy::frame::LazyFileListReader;
use tokio::sync::mpsc;

// use std::sync::Arc;
pub static LENS: OnceCell<Lens> = OnceCell::new();

pub fn get_state() -> State {
    LENS.get().unwrap().state.clone()
}

pub fn set_state_collection(collection: Collection) {
    let state = get_state();
    let mut state_collection = state.working_collection.lock().unwrap();
    *state_collection = collection;
}

pub fn get_state_collection() -> Collection {
    let state = get_state();
    let collection_mutex = state.working_collection.lock().unwrap();
    collection_mutex.clone()
}

pub fn set_state_dataset(dataset: Dataset) {
    let state = get_state();
    let mut state_dataset = state.working_dataset.lock().unwrap();
    *state_dataset = dataset;
}

pub fn get_state_dataset() -> Dataset {
    let state = get_state();
    let dataset_mutex = state.working_dataset.lock().unwrap();
    dataset_mutex.clone()
}

pub fn set_state_dataset_files(files: Vec<File>) {
    let state = get_state();
    let mut state_files = state.working_files.lock().unwrap();
    *state_files = files;
}
pub fn get_state_dataset_files() -> Vec<File> {
    let state = get_state();
    let state_files_mutex = state.working_files.lock().unwrap();
    state_files_mutex.clone()
}

pub fn add_or_get_progress(key: String) -> Arc<Mutex<SrPair>> {
    let state = get_state();

    let mut sr_map = state.progress_sr.lock().unwrap();

    sr_map
        .entry(key.clone())
        .or_insert(Arc::new(Mutex::new(mpsc::unbounded_channel())));

    sr_map.get(&key).unwrap().clone()
}

pub fn set_state_lfp_series() {
    use memmap2::MmapOptions;
    use ndarray::{s, Array2};

    let state = get_state();
    let filepath = state.working_directory.clone();
    let filepath = filepath.join("data/hc/hc-3/ec012ec.11/ec012ec.188/ec012ec.188.eeg");
    let file = std::fs::File::open(filepath).unwrap();
    let file_size = file.metadata().unwrap().len();
    let n_channels: usize = 33;
    let srate: usize = 1250;
    let n_samples: usize = (file_size as f64 / 2.0 / n_channels as f64) as usize;

    let mmo = MmapOptions::new();
    let mmap = unsafe { mmo.map(&file).unwrap() };

    // Interpret the memory-mapped file as a slice of i16
    let data: &[i16] =
        unsafe { std::slice::from_raw_parts(mmap.as_ptr() as *const i16, n_samples * n_channels) };

    // Convert the slice to a 2D array
    let array = Array2::from_shape_vec((n_samples, n_channels), data.to_vec()).unwrap();

    let s0 = 0;
    let sf = s0 + srate * 3;

    let ch: usize = 0;

    let slice = array.slice(s![s0..sf, ch]);

    let state = get_state();
    let mut series = state.lfp_series.lock().unwrap();
    *series = Vec::new();

    for (s, v) in slice.into_iter().enumerate() {
        series.push([s as f64, *v as f64]); //cannot index like thys
    }
}

pub fn set_state_spk_series() {
    use memmap2::MmapOptions;
    use ndarray::{s, Array3, Axis};

    let state = get_state();
    let filepath = state.working_directory.clone();
    let filepath = filepath.join("data/hc/hc-3/ec012ec.11/ec012ec.188/ec012ec.188.spk.1");
    let file = std::fs::File::open(filepath).unwrap();
    let file_size = file.metadata().unwrap().len();

    let n_channels: usize = 8;
    let n_samples: usize = 32;
    let n_spikes: usize = (file_size as f64 / 2.0 / n_samples as f64 / n_channels as f64) as usize;

    let mmo = MmapOptions::new();
    let mmap = unsafe { mmo.map(&file).unwrap() };

    let data: &[i16] = unsafe {
        std::slice::from_raw_parts(
            mmap.as_ptr() as *const i16,
            n_spikes * n_samples * n_channels,
        )
    };

    let array = Array3::from_shape_vec((n_spikes, n_samples, n_channels), data.to_vec()).unwrap();

    let slice = array.slice(s![0..100, .., ..]);

    let state = get_state();
    let mut series = state.spk_series.lock().unwrap();
    *series = Vec::new();

    for s in 0..100 {
        let spk_slice = slice.index_axis(Axis(0), s);
        let spk_slice = spk_slice.t();
        let mut spk_series = Vec::new();
        for (s, v) in spk_slice.into_iter().enumerate() {
            spk_series.push([s as f64, *v as f64]);
        }
        series.push(spk_series);
    }
}

pub fn set_state_fet_series() {
    use polars::prelude::LazyCsvReader;

    let state = get_state();
    let filepath = state.working_directory.clone();
    let filepath = filepath.join("data/hc/hc-3/ec012ec.11/ec012ec.188/ec012ec.188.fet.1");

    let file_df = LazyCsvReader::new(filepath)
        .with_has_header(false)
        .with_skip_rows(1)
        .with_separator(b' ')
        .finish()
        .unwrap()
        .collect()
        .unwrap();

    // std::iter::zip(file_df[0].iter(), file_df[1].iter()).map(|(a, b)| [a, b.cast(f64)]);

    println!("{}", file_df);
}
