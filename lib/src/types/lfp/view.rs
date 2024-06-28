struct LfpView {}

fn a() {
    use memmap2::MmapOptions;
    use ndarray::{s, Array2};
    use std::str::FromStr;

    let eeg_filepath =
        "/home/zegois/rust/ufrn/project/lens/data/hc/hc-3/ec012ec.11/ec012ec.188/ec012ec.188.eeg";
    let eeg_filepath = std::path::PathBuf::from_str(eeg_filepath).unwrap();
    let file = std::fs::File::open(eeg_filepath).unwrap();
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
    let mut series = state.series.lock().unwrap();
    *series = Vec::new();

    for (s, v) in slice.into_iter().enumerate() {
        series.push([s as f64, *v as f64]); //cannot index like thys
    }
}
