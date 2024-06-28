use crate::global;

use crate::gui::misc::toasts;
use crate::gui::panel::CollectionPanel;
use crate::gui::traits::View;

use crate::types::CRCNS;

use std::sync::Arc;

#[derive(Clone)]
pub struct Main {
    pub toasts: toasts::Toasts,
    pub is_visible: bool,
}

impl Default for Main {
    fn default() -> Self {
        let toasts = toasts::Toasts::new()
            .anchor(egui::Align2::RIGHT_BOTTOM, (10.0, 10.0))
            .direction(egui::Direction::TopDown);

        Main {
            toasts,
            is_visible: true,
        }
    }
}

impl Main {
    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let state = global::get_state();

        CollectionPanel::default().update(ctx, _frame);

        let layout = egui::Layout::top_down(egui::Align::Center);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.allocate_ui_with_layout(ui.available_size(), layout, |ui| {
                egui::ScrollArea::both().show(ui, |ui| {
                    ui.heading("CRCNS - Lens");

                    ui.label(state.working_directory.to_str().unwrap());

                    if ui.button("Print dataframe").clicked() {
                        global::set_state_fet_series();
                    }
                    if ui.button("Get CRCNS").clicked() {
                        let c = state.collections.clone();

                        tokio::spawn(async move {
                            CRCNS::get(c).await;
                        });
                    }
                    ui.label(format!(
                        "Reference count: {}",
                        Arc::strong_count(&state.collections)
                    ));

                    if ui.button("Persist CRCNS").clicked() {
                        let c = state.collections.clone();
                        let wd = state.working_directory.clone();
                        tokio::spawn(async move {
                            CRCNS::persist(c, wd).await;
                        });
                    };

                    if ui.button("Add toast").clicked() {
                        self.toasts.add(toasts::Toast {
                            text: "Hello, World".into(),
                            kind: toasts::ToastKind::Info,
                            options: toasts::ToastOptions::default()
                                .duration_in_seconds(3.0)
                                .show_progress(true)
                                .show_icon(true),
                        });
                    }

                    if ui.button("q").clicked() {
                        println!("Spawning");

                        let base_url = "https://portal.nersc.gov/project/crcns/download/index.php";
                        let url = url::Url::parse(base_url).unwrap();
                        let url = url.join("hc-3/filelist.txt").unwrap();
                        println!("{:?}", url.as_str());
                    }

                    ui.label(state.working_dataset.lock().unwrap().alias.clone());

                    if ui.button("Spike").clicked() {
                        global::set_state_spk_series();
                    }

                    egui_plot::Plot::new("spk_plot")
                        .allow_zoom(true)
                        .allow_drag(true)
                        .allow_scroll(true)
                        .legend(egui_plot::Legend::default())
                        .show(ui, |plot_ui| {
                            let series = state.spk_series.lock().unwrap().clone();
                            for serie in series.into_iter() {
                                plot_ui.line(egui_plot::Line::new(serie))
                            }
                        });

                    if ui.button("LFP").clicked() {
                        global::set_state_lfp_series();
                    }

                    egui_plot::Plot::new("lfp_plot")
                        .allow_zoom(true)
                        .allow_drag(true)
                        .allow_scroll(true)
                        .legend(egui_plot::Legend::default())
                        .show(ui, |plot_ui| {
                            let series = state.lfp_series.lock().unwrap().clone();
                            plot_ui.line(egui_plot::Line::new(series).name("LFP"))
                        });

                    self.toasts.show(ctx);
                });
            })
        });
    }
}
