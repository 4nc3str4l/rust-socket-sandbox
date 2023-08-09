use crate::structs::{AppState, Message};
use eframe::egui;
use egui::plot::{Line, PlotPoints};
use egui::Context;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{Receiver, Sender};

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
                                let _ = utnw_clone
                                    .send(Message::NewClient {
                                        id: (id),
                                        ip: editing_ip.to_string(),
                                    })
                                    .await;
                            });
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
                        for window in state.connection_window.iter_mut() {
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

            render_windows(ctx, app_state.clone(), utnw.clone());
            
            // Example of how I could render a plot
            egui::Window::new("Connection Statistics").resizable(true).show(ctx, |ui| {
                let n = 128;

                let line_points = (0..=n)
                    .map(|i| {
                        use std::f64::consts::TAU;
                        let x = egui::remap(i as f64, 0.0..=n as f64, -TAU..=TAU);
                        [x, x.sin()]
                    })
                    .collect::<Vec<_>>();

                let line = Line::new(line_points);

                egui::plot::Plot::new("connection_stats ").show(ui, |plot_ui| plot_ui.line(line));
            });

            ctx.request_repaint();
        })
    }
}
enum WindowAction {
    Disconnect(u8),
    UpdateMessage(u8, String),
    Send(Sender<Message>, Message),
}

fn render_windows(ctx: &Context, app_state: Arc<Mutex<AppState>>, ui_to_network: Sender<Message>) {
    let ui_to_network_clone = ui_to_network.clone();
    let mut actions = Vec::new();

    {
        let mut state = app_state.lock().unwrap();
        for window_index in 0..state.connection_window.len() {
            let window_id = state.connection_window[window_index].id.clone();
            let utn_for_send = ui_to_network_clone.clone();
            let utn_for_disconnect = ui_to_network_clone.clone();

            egui::Window::new(&window_id.to_string())
                .resizable(true)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Ip Address:");
                        ui.label(&state.connection_window[window_index].connection.url);
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
                            state.connection_window[window_index].connection.send_bytes,
                            state.connection_window[window_index]
                                .connection
                                .received_bytes
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
                                        &state.connection_window[window_index].connection.messages
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

                    ui.horizontal(|ui| {
                        if ui
                            .add_sized(
                                ui.available_size(),
                                egui::TextEdit::multiline(
                                    &mut state.connection_window[window_index]
                                        .connection
                                        .editing_message,
                                ),
                            )
                            .changed()
                        {
                            if ui.input(|ev| ev.key_pressed(egui::Key::Enter)) {
                                let msg = state.connection_window[window_index]
                                    .connection
                                    .editing_message
                                    .clone();
                                actions.push(WindowAction::UpdateMessage(window_id, msg.clone()));
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
                });
        }
    }

    process_window_actions(actions, app_state);
}

fn process_window_actions(actions: Vec<WindowAction>, app_state: Arc<Mutex<AppState>>) {
    // Process actions
    let mut state = app_state.lock().unwrap();
    for action in actions {
        match action {
            WindowAction::Disconnect(id) => {
                state.windows_to_remove.push(id);
            }
            WindowAction::UpdateMessage(id, msg) => {
                if let Some(window) = state.connection_window.iter_mut().find(|w| w.id == id) {
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
        .connection_window
        .retain(|window| !windows_to_remove.contains(&window.id));
    state.windows_to_remove.clear();
}
