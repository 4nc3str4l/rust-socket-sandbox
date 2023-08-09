#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod networking;
mod structs;
mod ui;

use std::sync::{Arc, Mutex};

use networking::network_processor;
use structs::{AppState, Message};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let (ui2ntx, mut ui2nrx) = mpsc::channel::<Message>(12);
    let (mut n2uitx, ntuirx) = mpsc::channel::<Message>(200);

    tokio::spawn(async move {
        network_processor(&mut ui2nrx, &mut n2uitx).await;
    });

    let app_state = Arc::new(Mutex::new(AppState::default()));
    let ui = ui::UI::new(app_state, ui2ntx, ntuirx);
    ui.run().await
}
