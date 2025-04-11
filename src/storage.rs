use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use serde_json;
use crate::models::FamilyTree;

pub fn get_data_dir() -> Option<PathBuf> {
    ProjectDirs::from("com", "ancestrum", "project-ancestrum")
        .map(|proj_dirs| proj_dirs.data_dir().to_path_buf())
}

pub fn save_tree(tree: &FamilyTree) -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = get_data_dir().ok_or("Could not determine data directory")?;
    fs::create_dir_all(&data_dir)?;

    let file_path = data_dir.join("family_tree.json");
    let json = serde_json::to_string_pretty(tree)?;
    fs::write(file_path, json)?;

    Ok(())
}

pub fn load_tree() -> Result<FamilyTree, Box<dyn std::error::Error>> {
    let data_dir = get_data_dir().ok_or("Could not determine data directory")?;
    let file_path = data_dir.join("family_tree.json");

    if !file_path.exists() {
        return Ok(FamilyTree::new());
    }

    let json = fs::read_to_string(file_path)?;
    let tree = serde_json::from_str(&json)?;

    Ok(tree)
} 