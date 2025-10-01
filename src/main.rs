use eframe::egui::{self};

mod app;
mod task;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "ARG TODO",
        native_options,
        Box::new(|creation_context| {
            creation_context.egui_ctx.set_theme(egui::Theme::Dark);
            Ok(Box::new(app::ToDoApp::new()))
        }),
    );
}
