use crate::gui::app;

use crate::types::State;

use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

#[derive(Clone)]
pub struct Lens {
    pub trt: Arc<Mutex<Runtime>>,
    pub gui: Arc<Mutex<app::Main>>,
    pub state: State,
}

impl Lens {
    pub fn new(trt: Arc<Mutex<Runtime>>) -> Self {
        let gui = Arc::new(Mutex::new(app::Main::default()));
        let state = State::default();
        Lens { trt, gui, state }
    }
}

impl eframe::App for Lens {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.gui.lock().unwrap().update(ctx, frame);
    }
}
