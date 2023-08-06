#[derive(Default)]
pub struct Connection {
    pub is_connected: bool,
    pub url: String,
    pub messages: Vec<String>,
    pub editing_message: String,
    pub send_bytes: u64,
    pub received_bytes: u64,
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
    pub id: String,
    pub is_open: bool,
    pub connection: Connection,
}

impl Default for ConnectionWindow {
    fn default() -> Self {
        Self {
            id: String::new(),
            is_open: true,
            connection: Connection::default(),
        }
    }
}

impl ConnectionWindow {
    pub fn new(id: String, url: String) -> Self {
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
}

impl AppState {
    pub fn insert_new_window(&mut self, url: String) {
        self.windows_open += 1;
        self.connection_window
            .push(ConnectionWindow::new(self.windows_open.to_string(), url));
    }
}
