#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod structs;
mod ui;

use std::sync::{Arc, Mutex};

use structs::AppState;

fn main() -> Result<(), eframe::Error> {
    let app_state = Arc::new(Mutex::new(AppState::default()));
    let ui = ui::UI::new(app_state);
    ui.run()
}
