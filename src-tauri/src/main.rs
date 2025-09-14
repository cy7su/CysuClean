// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cleaner;
mod error;
mod config;

use cleaner::CleanerService;
use config::AppConfig;
use tauri::{State, Window};
use std::sync::Arc;
use tokio::sync::Mutex;

type CleanerState = Arc<Mutex<CleanerService>>;

#[tauri::command]
async fn scan_system(cleaner: State<'_, CleanerState>) -> Result<serde_json::Value, String> {
    let mut service = cleaner.lock().await;
    service.scan_system()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn clean_system(
    cleaner: State<'_, CleanerState>,
    categories: Vec<String>,
) -> Result<serde_json::Value, String> {
    let mut service = cleaner.lock().await;
    service.clean_categories(categories)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_scan_progress(cleaner: State<'_, CleanerState>) -> Result<f64, String> {
    let service = cleaner.lock().await;
    Ok(service.get_scan_progress())
}

#[tauri::command]
async fn get_clean_progress(cleaner: State<'_, CleanerState>) -> Result<f64, String> {
    let service = cleaner.lock().await;
    Ok(service.get_clean_progress())
}

#[tauri::command]
async fn minimize_window(window: Window) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())
}

#[tauri::command]
async fn maximize_window(window: Window) -> Result<(), String> {
    if window.is_maximized().unwrap_or(false) {
        window.unmaximize().map_err(|e| e.to_string())
    } else {
        window.maximize().map_err(|e| e.to_string())
    }
}

#[tauri::command]
async fn close_window(window: Window) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}

fn main() {
    env_logger::init();
    
    let config = AppConfig::load().unwrap_or_else(|_| AppConfig::default());
    let cleaner_service = Arc::new(Mutex::new(CleanerService::new(config)));

    tauri::Builder::default()
        .manage(cleaner_service)
        .invoke_handler(tauri::generate_handler![
            scan_system,
            clean_system,
            get_scan_progress,
            get_clean_progress,
            minimize_window,
            maximize_window,
            close_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
