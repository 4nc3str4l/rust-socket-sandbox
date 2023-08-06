use crate::structs::{AppState, Message, Operation};
use eframe::egui;
use egui::Context;
use tokio::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, Mutex};

pub struct UI {
    pub app_state: Arc<Mutex<AppState>>,
    pub ui_to_network: Sender<Message>,
    pub network_to_ui: Receiver<Message>
}

impl UI {
    pub fn new(app_state: Arc<Mutex<AppState>>, uitn: Sender<Message>, ntui: Receiver<Message>) -> Self {
        Self { 
            app_state,
            ui_to_network: uitn,
            network_to_ui: ntui
        }
    }

    pub fn run(&self) -> Result<(), eframe::Error> {
        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(1280., 920.)),
            ..Default::default()
        };

        let app_state = Arc::clone(&self.app_state);
        let utnw = self.ui_to_network.clone(); 

        eframe::run_simple_native("Rust Socket Sandbox", options, move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |_ui| {
                egui::Window::new("Connection Manager").show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        let mut state = app_state.lock().unwrap();
                        ui.label("Ip Address:");
                        ui.text_edit_singleline(&mut state.editing_ip);
                        if ui.button("Create Connection").clicked() {
                            let editing_ip = state.editing_ip.clone();
                            let id = state.insert_new_window(editing_ip.to_owned());
                            let utnw_clone = utnw.clone();
                            tokio::spawn(async move {
                                let _ = utnw_clone.send(Message { id: id.to_string(), operation: Operation::NewClient, payload: editing_ip.to_owned() }).await;
                            });
                        }
                    });
                });
            });

            Self::render_windows(ctx, &app_state);
            ctx.request_repaint();
        })
    }

    fn render_windows(ctx: &Context, app_state: &Arc<Mutex<AppState>>) {
        let mut state = app_state.lock().unwrap();
        for connection_window in &mut state.connection_window {
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
                        connection_window
                            .connection
                            .messages
                            .push(connection_window.connection.editing_message.clone());
                        connection_window.connection.editing_message.clear();
                    }
                });
            });
        }
    }
}
