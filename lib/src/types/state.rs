use crate::types::Collection;
use crate::types::Dataset;
use crate::types::File;

use std::collections::HashMap;
use std::collections::HashSet;
use std::env::current_dir;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;

pub type SrPair = (UnboundedSender<f32>, UnboundedReceiver<f32>);
pub type SrMap = HashMap<String, Arc<Mutex<SrPair>>>;

#[derive(Clone)]
pub struct State {
    pub working_files: Arc<Mutex<Vec<File>>>,
    pub working_dataset: Arc<Mutex<Dataset>>,
    pub working_collection: Arc<Mutex<Collection>>,
    pub working_directory: PathBuf,

    pub collections: Arc<Mutex<Vec<Collection>>>,

    pub progress_sr: Arc<Mutex<SrMap>>,
    pub progress: Arc<Mutex<HashMap<String, f32>>>,
    pub progress_done: Arc<Mutex<HashSet<String>>>,

    pub lfp_series: Arc<Mutex<Vec<[f64; 2]>>>,
    pub spk_series: Arc<Mutex<Vec<Vec<[f64; 2]>>>>,
    pub fet_series: Arc<Mutex<Vec<[f64; 2]>>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            lfp_series: Arc::new(Mutex::new(Vec::new())),
            spk_series: Arc::new(Mutex::new(Vec::new())),
            fet_series: Arc::new(Mutex::new(Vec::new())),

            working_files: Arc::new(Mutex::new(Vec::new())),
            working_dataset: Arc::new(Mutex::new(Dataset::default())),
            working_collection: Arc::new(Mutex::new(Collection::default())),
            working_directory: current_dir().unwrap(),

            collections: Arc::new(Mutex::new(Vec::new())),

            progress: Arc::new(Mutex::new(HashMap::new())),
            progress_sr: Arc::new(Mutex::new(HashMap::new())),
            progress_done: Arc::new(Mutex::new(HashSet::new())),
        }
    }
}
