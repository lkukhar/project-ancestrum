use std::fs;
use std::path::Path;
use crate::models::FamilyTree;
use crate::commands::CommandResult;

pub struct Storage {
    data_dir: String,
}

impl Storage {
    pub fn new(data_dir: String) -> Self {
        if !Path::new(&data_dir).exists() {
            fs::create_dir_all(&data_dir).expect("Failed to create data directory");
        }
        Self { data_dir }
    }

    pub fn save_tree(&self, tree: &FamilyTree) -> CommandResult<()> {
        let file_path = format!("{}/{}.json", self.data_dir, tree.id);
        match serde_json::to_string_pretty(tree) {
            Ok(json) => {
                if let Err(e) = fs::write(&file_path, json) {
                    return CommandResult::error(format!("Failed to write file: {}", e));
                }
                CommandResult::success(())
            }
            Err(e) => CommandResult::error(format!("Failed to serialize tree: {}", e)),
        }
    }

    pub fn load_tree(&self, tree_id: &str) -> CommandResult<FamilyTree> {
        let file_path = format!("{}/{}.json", self.data_dir, tree_id);
        match fs::read_to_string(&file_path) {
            Ok(json) => {
                match serde_json::from_str(&json) {
                    Ok(tree) => CommandResult::success(tree),
                    Err(e) => CommandResult::error(format!("Failed to parse tree: {}", e)),
                }
            }
            Err(e) => CommandResult::error(format!("Failed to read file: {}", e)),
        }
    }
} 