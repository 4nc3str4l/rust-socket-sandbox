#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod structs;
mod ui;
mod networking;

use std::sync::{Arc, Mutex};

use networking::packet_processor;
use structs::{AppState, Message};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let (ui2ntx, ui2nrx)  = mpsc::channel::<Message>(12);
    let (n2uitx, ntuirx)  = mpsc::channel::<Message>(200);

    packet_processor().await;
    let app_state = Arc::new(Mutex::new(AppState::default()));
    let ui = ui::UI::new(app_state, ui2ntx, ntuirx);
    ui.run()
}
