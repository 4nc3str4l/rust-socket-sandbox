use tokio::sync::mpsc::Sender;

#[derive(Default)]
pub struct Connection {
    pub is_connected: bool,
    pub url: String,
    pub messages: Vec<String>,
    pub editing_message: String,
    pub send_bytes: usize,
    pub received_bytes: usize,
}

impl Connection {
    pub fn new(url: String) -> Self {
        Self {
            is_connected: false,
            url,
            messages: Vec::new(),
            editing_message: String::new(),
            send_bytes: 0,
            received_bytes: 0,
        }
    }
}

pub struct ConnectionWindow {
    pub id: u8,
    pub is_open: bool,
    pub connection: Connection,
    pub send_option: SendOptions,
}

impl Default for ConnectionWindow {
    fn default() -> Self {
        Self {
            id: 0,
            is_open: true,
            connection: Connection::default(),
            send_option: SendOptions::Manual,
        }
    }
}

impl ConnectionWindow {
    pub fn new(id: u8, url: String) -> Self {
        Self {
            id,
            is_open: true,
            connection: Connection::new(url),
            send_option: SendOptions::Manual,
        }
    }
}

#[derive(Default)]
pub struct AppState {
    pub connections: Vec<ConnectionWindow>,
    pub editing_ip: String,
    pub windows_open: u8,
    pub windows_to_remove: Vec<u8>,
}

impl AppState {
    pub fn insert_new_window(&mut self, url: String) -> u8 {
        self.windows_open += 1;
        self.connections
            .push(ConnectionWindow::new(self.windows_open, url));
        return self.windows_open;
    }
}

#[derive(Debug)]
pub enum Message {
    NewClient { id: u8, ip: String },
    Message { id: u8, payload: String, num_bytes: usize },
    Close { id: u8 },
}

#[derive(PartialEq)]
pub enum SendOptions {
    Periodically,
    Random,
    Manual,
    File,
    N,
}

pub enum WindowAction {
    Disconnect(u8),
    UpdateMessage(u8, String),
    Send(Sender<Message>, Message),
}
