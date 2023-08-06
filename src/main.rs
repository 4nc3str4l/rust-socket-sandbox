#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod structs;
mod ui;

use structs::AppState;
use ui::run_ui;

fn main() -> Result<(), eframe::Error> {
    let app_state = AppState::default();
    run_ui(app_state)
}
