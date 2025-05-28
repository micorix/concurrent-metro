mod cmd;
mod state;

mod config;

mod train_thread;
mod common;
mod ipc;
// mod emitter_thread;

use crate::cmd::{start_threads, stop_all_threads, read_config};
use crate::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![start_threads, stop_all_threads, read_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
