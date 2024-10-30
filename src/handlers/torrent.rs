use std::time::{SystemTime, UNIX_EPOCH};

pub struct Torrent {
    title: String,
    started: u128,
    peers: u32,
    seeders: u32,
    completed: u128,
    total: u128
}

impl Torrent {

    pub fn new(title: &str) -> Self {
        let started = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();
        Self {
            title: title.to_string(),
            started,
            peers: 0,
            seeders: 0,
            completed: 0,
            total: 0
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_peers(&self) -> u32 {
        self.peers
    }

    pub fn get_seeders(&self) -> u32 {
        self.seeders
    }

    pub fn get_progress(&self) -> u128 {
        self.completed
    }

    pub fn get_total(&self) -> u128 {
        self.total
    }

    pub fn get_left(&self) -> u128 {
        self.total-self.completed
    }

    pub fn get_started(&self) -> u128 {
        self.started
    }
}
