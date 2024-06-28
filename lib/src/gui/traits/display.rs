pub trait Display {
    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Is the demo enabled for this integration?
    fn is_enabled(&self, _ctx: &egui::Context) -> bool {
        true
    }

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}
