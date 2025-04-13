use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use serde_json;
use crate::models::FamilyTree;

pub fn get_data_dir() -> PathBuf {
    let project_dirs = ProjectDirs::from("com", "ancestrum", "ancestrum")
        .expect("Failed to get project directories");
    let data_dir = project_dirs.data_dir();
    fs::create_dir_all(data_dir).expect("Failed to create data directory");
    data_dir.to_path_buf()
}

pub fn save_tree(tree: &FamilyTree) -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = get_data_dir();
    let file_path = data_dir.join("family_tree.json");
    let json = serde_json::to_string_pretty(tree)?;
    fs::write(file_path, json)?;
    Ok(())
}

pub fn load_tree() -> Result<FamilyTree, Box<dyn std::error::Error>> {
    let data_dir = get_data_dir();
    let file_path = data_dir.join("family_tree.json");
    
    if !file_path.exists() {
        return Ok(FamilyTree::new());
    }
    
    let json = fs::read_to_string(file_path)?;
    let tree = serde_json::from_str(&json)?;
    Ok(tree)
} 