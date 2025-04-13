#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod models;
mod storage;

use commands::AppState;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            tree: std::sync::Mutex::new(storage::load_tree().unwrap_or_else(|_| models::family_tree::FamilyTree::new())),
        })
        .invoke_handler(tauri::generate_handler![
            commands::add_person,
            commands::get_root_person,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 