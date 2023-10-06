use crate::structs::{AppState, Message, SendOptions, WindowAction};
use eframe::egui;
use egui::{CollapsingHeader, Context, Resize};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{Receiver, Sender};
use crate::utils::is_valid_websocket_ip;

pub struct UI {
    pub app_state: Arc<Mutex<AppState>>,
    pub network_to_ui: Arc<Mutex<Receiver<Message>>>,
    pub ui_to_network: Sender<Message>,
}

impl UI {
    pub fn new(
        app_state: Arc<Mutex<AppState>>,
        uitn: Sender<Message>,
        ntui: Receiver<Message>,
    ) -> Self {
        Self {
            app_state,
            ui_to_network: uitn,
            network_to_ui: Arc::new(Mutex::new(ntui)),
        }
    }

    pub async fn run(&self) -> Result<(), eframe::Error> {
        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(1280., 720.)),
            ..Default::default()
        };

        let app_state = Arc::clone(&self.app_state);
        let utnw = self.ui_to_network.clone();
        let ntui = Arc::clone(&self.network_to_ui);

        let mut should_create_connection = false;

        eframe::run_simple_native("Rust Socket Sandbox", options, move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |_ui| {
                egui::Window::new("Connection Manager").show(ctx, |ui| {
                    let mut state = app_state.lock().unwrap();
                    if state.in_error {
                        ui.horizontal(|ui| {
                            ui.centered_and_justified(|ui| {
                                ui.colored_label(egui::Color32::RED, "Invalid IP Address");
                            });
                        });
                    }
                    ui.horizontal(|ui| {
                        ui.label("Ip Address:");
                        ui.text_edit_singleline(&mut state.editing_ip);
                        if ui.button("Create Connection").clicked() {
                            if is_valid_websocket_ip(&state.editing_ip) {
                                state.in_error = false;
                                should_create_connection = true && !state.editing_ip.is_empty();
                            } else {
                                state.in_error = true;
                            }

                        }
                    });
                });
            });

            let mut ntui_lock = ntui.lock().unwrap();
            while let Ok(message) = ntui_lock.try_recv() {
                match message {
                    Message::NewClient { id, ip } => todo!(),
                    Message::Message {
                        id,
                        payload,
                        num_bytes,
                    } => {
                        let mut state = app_state.lock().unwrap();
                        for window in state.connections.iter_mut() {
                            if window.id == id {
                                window.connection.messages.push(payload.to_owned());
                                window.connection.received_bytes += num_bytes;
                                break;
                            }
                        }
                        println!("Processing a message = {:?}", &payload);
                    }
                    Message::Close { id } => todo!(),
                }
            }

            if should_create_connection {
                create_connection(app_state.clone(), utnw.clone());
                should_create_connection = false;
            }

            render_windows(ctx, app_state.clone(), utnw.clone());

            ctx.request_repaint();
        })
    }
}

pub fn create_connection(app_state: Arc<Mutex<AppState>>, ui_to_network: Sender<Message>) {
    let mut state = app_state.lock().unwrap();
    let editing_ip = state.editing_ip.clone();
    let id = state.insert_new_window(editing_ip.to_owned());
    let utnw_clone = ui_to_network.clone();
    tokio::spawn(async move {
        let _ = utnw_clone
            .send(Message::NewClient {
                id: (id),
                ip: editing_ip.to_string(),
            })
            .await;
    });
}

fn render_windows(ctx: &Context, app_state: Arc<Mutex<AppState>>, ui_to_network: Sender<Message>) {
    let ui_to_network_clone = ui_to_network.clone();
    let mut actions = Vec::new();

    {
        let mut state = app_state.lock().unwrap();
        for window_index in 0..state.connections.len() {
            let window_id = state.connections[window_index].id.clone();
            let utn_for_send = ui_to_network_clone.clone();
            let utn_for_disconnect = ui_to_network_clone.clone();

            egui::Window::new(&window_id.to_string())
                .resizable(true)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Ip Address:");
                        ui.label(&state.connections[window_index].connection.url);
                        if ui.button("Disconnect").clicked() {
                            actions.push(WindowAction::Disconnect(window_id));
                            actions.push(WindowAction::Send(
                                utn_for_disconnect,
                                Message::Close {
                                    id: window_id.clone(),
                                },
                            ));
                        }
                    });

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "Sent / Recv [{} / {}] bytes",
                            state.connections[window_index].connection.send_bytes,
                            state.connections[window_index].connection.received_bytes
                        ));
                    });

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Messages:");
                        egui::ScrollArea::vertical()
                            .min_scrolled_height(400.)
                            .stick_to_bottom(true)
                            .show(ui, |ui| {
                                ui.vertical(|ui| {
                                    for message in
                                        &state.connections[window_index].connection.messages
                                    {
                                        ui.horizontal(|ui| {
                                            if ui
                                                .button("📋")
                                                .on_hover_text("Click to copy")
                                                .clicked()
                                            {
                                                ui.output_mut(|o| {
                                                    o.copied_text = message.to_string()
                                                });
                                            }
                                            ui.add(egui::Label::new(message).wrap(true));
                                        });
                                    }
                                });
                            });
                    });

                    ui.separator();

                    ui.label("Send Options:");
                    ui.horizontal(|ui| {
                        ui.radio_value(
                            &mut state.connections[window_index].send_option,
                            SendOptions::Manual,
                            "Manual",
                        );
                        ui.radio_value(
                            &mut state.connections[window_index].send_option,
                            SendOptions::Periodically,
                            "Periodically",
                        );
                        ui.radio_value(
                            &mut state.connections[window_index].send_option,
                            SendOptions::Random,
                            "Random",
                        );
                        ui.radio_value(
                            &mut state.connections[window_index].send_option,
                            SendOptions::File,
                            "File",
                        );
                        ui.radio_value(
                            &mut state.connections[window_index].send_option,
                            SendOptions::N,
                            "N",
                        );
                    });

                    match state.connections[window_index].send_option {
                        SendOptions::Periodically => {}
                        SendOptions::Random => {}
                        SendOptions::Manual => {
                            ui.horizontal(|ui| {
                                if ui
                                    .add_sized(
                                        ui.available_size(),
                                        egui::TextEdit::multiline(
                                            &mut state.connections[window_index]
                                                .connection
                                                .editing_message,
                                        ),
                                    )
                                    .changed()
                                {
                                    if ui.input(|ev| ev.key_pressed(egui::Key::Enter)) {
                                        let msg = state.connections[window_index]
                                            .connection
                                            .editing_message
                                            .clone();
                                        actions.push(WindowAction::UpdateMessage(
                                            window_id,
                                            msg.clone(),
                                        ));
                                        actions.push(WindowAction::Send(
                                            utn_for_send,
                                            Message::Message {
                                                id: window_id,
                                                payload: msg,
                                                num_bytes: 0,
                                            },
                                        ));
                                    }
                                }
                            });
                        }
                        SendOptions::File => {}
                        SendOptions::N => {}
                    }
                });
        }
    }

    // Process actions
    let mut state = app_state.lock().unwrap();
    for action in actions {
        match action {
            WindowAction::Disconnect(id) => {
                state.windows_to_remove.push(id);
            }
            WindowAction::UpdateMessage(id, msg) => {
                if let Some(window) = state.connections.iter_mut().find(|w| w.id == id) {
                    window.connection.messages.push(msg.clone());
                    window.connection.editing_message.clear();
                    window.connection.send_bytes += msg.as_bytes().len();
                }
            }
            WindowAction::Send(sender, message) => {
                tokio::spawn(async move {
                    let _ = sender.send(message).await;
                });
            }
        }
    }

    let windows_to_remove = state.windows_to_remove.clone();
    state
        .connections
        .retain(|window| !windows_to_remove.contains(&window.id));
    state.windows_to_remove.clear();
}
