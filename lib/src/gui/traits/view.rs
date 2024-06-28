pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
}
