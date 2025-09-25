use jiff::{Zoned, civil::DateTime};
use std::{fs, io::ErrorKind};

mod app;
mod task;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "ARG TODO",
        native_options,
        Box::new(|cc| Ok(Box::new(app::ToDoApp::new(cc)))),
    );
}