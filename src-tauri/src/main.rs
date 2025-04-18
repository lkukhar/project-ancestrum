#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod api;
mod models;

use api::handlers::*;
use models::FamilyTree;
use std::sync::Mutex;
use tauri::State;

pub struct AppState {
    pub family_tree: Mutex<FamilyTree>,
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            family_tree: Mutex::new(FamilyTree::new()),
        })
        .invoke_handler(tauri::generate_handler![
            get_tree,
            add_person,
            get_person,
            update_person,
            delete_person,
            add_relationship
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 