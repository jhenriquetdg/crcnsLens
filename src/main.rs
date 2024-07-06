use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::time::sleep;

use lib::global::LENS;
use lib::gui::app::Lens;

fn main() -> eframe::Result<()> {
    let trt = Runtime::new().expect("Unable to initialize tokio runtime.");
    let trt = Arc::new(Mutex::new(trt));

    let t = trt.clone();
    let _enter = t.lock().unwrap().enter();
    std::thread::spawn(move || {
        t.lock().unwrap().block_on(async {
            loop {
                sleep(Duration::from_secs(3600)).await;
            }
        })
    });
    let app = Lens::new(trt.clone());

    // std::thread::spawn(move || async {
    //     loop {
    //         sleep(Duration::from_secs(3)).await;
    //         let state = lib::global::get_state();
    //         for (k, v) in state.progress_sr.lock().unwrap().into_iter() {
    //             let p = v.lock().unwrap().1.recv().await.unwrap();
    //             if p == 1.0 {
    //                 state.progress_done.lock().unwrap().insert(k);
    //             } else {
    //                 state.progress.lock().unwrap().entry(k).or_insert(p);
    //             };
    //         }
    //     }
    // });

    let data_directory = app.state.working_directory.join("data");
    std::fs::create_dir_all(data_directory).expect("Unable to create data directory.");

    LENS.get_or_init(|| app.clone());

    eframe::run_native(
        "CRCNS - Lens",
        eframe::NativeOptions::default(),
        Box::new(|_cc| {
            println!("Running CRCNS Lens");
            Box::new(app)
        }),
    )
}
