use std::sync::Mutex;

mod models;

struct AppState {
    family_tree: Mutex<models::FamilyTree>,
}

#[tauri::command]
fn add_person(state: tauri::State<AppState>, name: String) -> Result<String, String> {
    let mut tree = state.family_tree.lock().unwrap();
    tree.add_person(name)
        .map(|id| id.to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_person(state: tauri::State<AppState>, id: String) -> Result<models::Person, String> {
    let tree = state.family_tree.lock().unwrap();
    tree.get_person(&id)
        .cloned()
        .ok_or_else(|| "Person not found".to_string())
}

#[tauri::command]
fn update_person(
    state: tauri::State<AppState>,
    id: String,
    name: String,
    birth_date: Option<String>,
    death_date: Option<String>,
    gender: String,
    notes: String,
) -> Result<(), String> {
    let mut tree = state.family_tree.lock().unwrap();
    let gender = match gender.as_str() {
        "Male" => models::Gender::Male,
        "Female" => models::Gender::Female,
        _ => models::Gender::Other,
    };
    tree.update_person(&id, name, birth_date, death_date, gender, notes)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_person(state: tauri::State<AppState>, id: String) -> Result<(), String> {
    let mut tree = state.family_tree.lock().unwrap();
    tree.delete_person(&id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn add_relationship(
    state: tauri::State<AppState>,
    from_id: String,
    to_id: String,
    relationship: String,
) -> Result<(), String> {
    let mut tree = state.family_tree.lock().unwrap();
    let relationship = match relationship.as_str() {
        "Parent" => models::Relationship::Parent,
        "Child" => models::Relationship::Child,
        "Sibling" => models::Relationship::Sibling,
        "Spouse" => models::Relationship::Spouse,
        _ => return Err("Invalid relationship type".to_string()),
    };
    tree.add_relationship(&from_id, &to_id, relationship)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_relationships(state: tauri::State<AppState>, id: String) -> Result<Vec<(String, String)>, String> {
    let tree = state.family_tree.lock().unwrap();
    tree.get_relationships(&id)
        .map_err(|e| e.to_string())
}

pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            family_tree: Mutex::new(models::FamilyTree::new()),
        })
        .invoke_handler(tauri::generate_handler![
            add_person,
            get_person,
            update_person,
            delete_person,
            add_relationship,
            get_relationships,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
