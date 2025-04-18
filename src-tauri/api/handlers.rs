use crate::models::{FamilyTree, Person, Relationship};
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub status: u16,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T, message: &str) -> Self {
        Self {
            data,
            message: message.to_string(),
        }
    }
}

#[tauri::command]
pub async fn get_tree(state: State<'_, FamilyTree>) -> Result<ApiResponse<FamilyTree>, ApiError> {
    Ok(ApiResponse::new(state.clone(), "Tree retrieved successfully"))
}

#[tauri::command]
pub async fn add_person(
    state: State<'_, FamilyTree>,
    person: Person,
) -> Result<ApiResponse<Person>, ApiError> {
    let id = Uuid::new_v4().to_string();
    let mut person = person;
    person.id = id.clone();
    
    state.add_person(person.clone());
    Ok(ApiResponse::new(person, "Person added successfully"))
}

#[tauri::command]
pub async fn update_person(
    state: State<'_, FamilyTree>,
    id: String,
    person: Person,
) -> Result<ApiResponse<Person>, ApiError> {
    if let Some(updated_person) = state.update_person(&id, person.clone()) {
        Ok(ApiResponse::new(updated_person, "Person updated successfully"))
    } else {
        Err(ApiError {
            message: "Person not found".to_string(),
            status: 404,
        })
    }
}

#[tauri::command]
pub async fn delete_person(
    state: State<'_, FamilyTree>,
    id: String,
) -> Result<ApiResponse<()>, ApiError> {
    if state.delete_person(&id) {
        Ok(ApiResponse::new((), "Person deleted successfully"))
    } else {
        Err(ApiError {
            message: "Person not found".to_string(),
            status: 404,
        })
    }
}

#[tauri::command]
pub async fn add_relationship(
    state: State<'_, FamilyTree>,
    from_id: String,
    to_id: String,
    relationship_type: Relationship,
) -> Result<ApiResponse<()>, ApiError> {
    if state.add_relationship(&from_id, &to_id, relationship_type) {
        Ok(ApiResponse::new((), "Relationship added successfully"))
    } else {
        Err(ApiError {
            message: "Failed to add relationship".to_string(),
            status: 400,
        })
    }
} 