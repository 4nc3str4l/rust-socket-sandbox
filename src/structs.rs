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
}

impl Default for ConnectionWindow {
    fn default() -> Self {
        Self {
            id: 0,
            is_open: true,
            connection: Connection::default(),
        }
    }
}

impl ConnectionWindow {
    pub fn new(id: u8, url: String) -> Self {
        Self {
            id,
            is_open: true,
            connection: Connection::new(url),
        }
    }
}

#[derive(Default)]
pub struct AppState {
    pub connection_window: Vec<ConnectionWindow>,
    pub editing_ip: String,
    pub windows_open: u8,
    pub windows_to_remove: Vec<u8>,
}

impl AppState {
    pub fn insert_new_window(&mut self, url: String) -> u8 {
        self.windows_open += 1;
        self.connection_window
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
