use tauri::State;
use crate::models::{FamilyTree, Person, Relationship};
use crate::AppState;
use std::sync::Mutex;
use std::path::PathBuf;
use dirs;

const DATA_FILE: &str = "family_tree.json";

#[tauri::command]
pub fn get_tree(state: State<AppState>) -> FamilyTree {
    let tree = state.family_tree.lock().unwrap();
    (*tree).clone()
}

#[tauri::command]
pub fn add_person(state: State<AppState>, person: Person) -> Result<Person, String> {
    let mut tree = state.family_tree.lock().unwrap();
    let result = tree.add_person(person)?;
    
    // Save the tree after adding a person
    let home_dir = dirs::data_dir()
        .ok_or_else(|| "Failed to get data directory".to_string())?;
    let app_data_dir = home_dir.join("Ancestrum");
    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    let data_path = app_data_dir.join(DATA_FILE);
    tree.save_to_file(&data_path)?;
    
    Ok(result)
}

#[tauri::command]
pub fn get_person(state: State<AppState>, id: String) -> Result<Person, String> {
    let tree = state.family_tree.lock().unwrap();
    tree.get_person(&id)
        .map(|p| p.clone())
        .ok_or_else(|| format!("Person with id {} not found", id))
}

#[tauri::command]
pub fn update_person(state: State<AppState>, id: String, person: Person) -> Result<Person, String> {
    let mut tree = state.family_tree.lock().unwrap();
    tree.update_person(&id, person)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_person(state: State<AppState>, id: String) -> Result<(), String> {
    let mut tree = state.family_tree.lock().unwrap();
    tree.delete_person(&id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_relationship(
    state: State<AppState>,
    from_id: String,
    to_id: String,
    relationship_type: Relationship,
) -> Result<(), String> {
    let mut tree = state.family_tree.lock().unwrap();
    tree.add_relationship(&from_id, &to_id, relationship_type)
        .map_err(|e| e.to_string())
} 