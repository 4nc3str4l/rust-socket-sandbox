#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod structs;

use eframe::egui;
use structs::{ConnectionWindow, AppState};



fn main() -> Result<(), eframe::Error> {

    let mut app_state = AppState::default();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280., 920.)),
        ..Default::default()
    };

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Window::new("Connection Manager").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Ip Address:");
                    ui.text_edit_singleline(&mut app_state.editing_ip);
                    if ui.button("Create Connection").clicked() {
                        app_state.insert_new_window(app_state.editing_ip.clone());
                    }
                });
            });
        });

        for connection_window in &mut app_state.connection_window {
            egui::Window::new(&connection_window.id).show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Ip Address:");
                    ui.label(&connection_window.connection.url);
                    if ui.button("Connect").clicked() {
                        connection_window.connection.is_connected = true;
                    }
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Messages:");
                    ui.vertical(|ui| {
                        for message in &connection_window.connection.messages {
                            ui.label(message);
                        }
                    });
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.text_edit_multiline(&mut connection_window.connection.editing_message);
                    if ui.button("Send").clicked() {
                        connection_window.connection.messages.push(connection_window.connection.editing_message.clone());
                        connection_window.connection.editing_message.clear();
                    }
                });
            });
        }        
    })
}