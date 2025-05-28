use std::sync::{Arc, Mutex, mpsc::Sender, atomic::{AtomicBool, Ordering}};
use crate::config::MetroConfig;

pub struct AppState {
    pub message_sender: Mutex<Option<Sender<String>>>,
    pub stop_flag: Arc<AtomicBool>,
    pub config: Arc<Mutex<MetroConfig>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            message_sender: Mutex::new(None),
            stop_flag: Arc::new(AtomicBool::new(false)),
            config: Arc::new(Mutex::new(MetroConfig::empty()))
        }
    }
}