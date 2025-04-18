use crate::State;
use crate::models::{FamilyTree, Person, Relationship};
use crate::AppState;

#[tauri::command]
pub fn get_tree(state: State<AppState>) -> FamilyTree {
    let tree = state.family_tree.lock().unwrap();
    (*tree).clone()
}

#[tauri::command]
pub fn add_person(state: State<AppState>, person: Person) -> Result<Person, String> {
    let mut tree = state.family_tree.lock().unwrap();
    tree.add_person(person)
        .map_err(|e| e.to_string())
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