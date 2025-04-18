#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod models;
mod handlers;

use models::FamilyTree;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.set_title("Ancestrum").unwrap();
            Ok(())
        })
        .manage(FamilyTree::new())
        .invoke_handler(tauri::generate_handler![
            handlers::get_tree,
            handlers::add_person,
            handlers::update_person,
            handlers::delete_person,
            handlers::add_relationship,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 