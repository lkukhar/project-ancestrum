use crate::models::family_tree::FamilyTree;
use crate::storage::{load_tree, save_tree};
use tauri::State;
use std::sync::Mutex;

pub struct AppState {
    pub tree: Mutex<FamilyTree>,
}

#[tauri::command]
pub fn add_person(state: State<AppState>, name: String) -> Result<(), String> {
    let mut tree = state.tree.lock().map_err(|e| e.to_string())?;
    let person = crate::models::person::Person::new(name);
    tree.add_person(person);
    save_tree(&tree).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_root_person(state: State<AppState>) -> Result<Option<String>, String> {
    let tree = state.tree.lock().map_err(|e| e.to_string())?;
    Ok(tree.get_root().map(|p| p.name.clone()))
} 