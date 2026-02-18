#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use slint::Model;
use std::error::Error;

slint::include_modules!();
fn main() -> Result<(), Box<dyn Error>> {
    let main_window = MainWindow::new()?;

    main_window.run()?;

    Ok(())
}
