#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod structs;
mod ui;
mod networking;

use std::sync::{Arc, Mutex};

use networking::client_spawner;
use structs::{AppState, Message};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let (mut ui2ntx, mut ui2nrx)  = mpsc::channel::<Message>(12);
    let (mut n2uitx, mut ntuirx)  = mpsc::channel::<Message>(200);

    tokio::spawn(async move  {
        client_spawner(&mut ui2nrx, &mut n2uitx).await;
    });
    
    let app_state = Arc::new(Mutex::new(AppState::default()));
    let ui = ui::UI::new(app_state, ui2ntx, ntuirx);
    ui.run()
}
