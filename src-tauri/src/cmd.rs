use crate::state::AppState;
use std::sync::{Arc};
use std::sync::atomic::{Ordering};
use tauri::{command, Manager, State, AppHandle, Emitter};
use tauri_plugin_opener;
use crate::config::{MetroConfig};
use crate::common::coords_lock::CoordinatesLock;
use crate::train_thread::Train;

#[command]
pub fn start_threads(app_handle: tauri::AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    state.stop_flag.store(false, Ordering::Relaxed);
    let coords_lock = Arc::new(CoordinatesLock::new());
    let stop_flag = state.stop_flag.clone(); // ✅ clone Arc once

    let update_fn = Arc::new(move |action| {
        app_handle.emit("dispatch_display_state_action", action).unwrap();
    });

    state.config.lock().unwrap().trains.iter().for_each(|train_config| {
        Train::new(
            train_config.clone(),
            stop_flag.clone(),
            coords_lock.clone(),
            Some(update_fn.clone())
        )
            .start();
    });

    Ok(())
}

#[command]
pub fn stop_all_threads(state: State<'_, AppState>) -> Result<(), String> {
    state.stop_flag.store(true, Ordering::Relaxed);
    println!("All threads have been signaled to stop.");
    Ok(())
}
#[command]
pub fn read_config(app_handle: AppHandle, file_path: String) -> Result<MetroConfig, String> {
    let config = MetroConfig::from_file(&file_path).map_err(|err| err.to_string())?;
    let json_config = config.to_json().map_err(|err| err.to_string())?;
    println!("Config JSON: {}", json_config);

    let state = app_handle.state::<AppState>();
    {
        let mut config_lock = state.config.lock().map_err(|_| "Failed to lock config")?;
        *config_lock = config.clone();
    }

    Ok(config)
}