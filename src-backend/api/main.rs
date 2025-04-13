mod models;
mod storage;

use models::FamilyTree;
use storage::{load_tree, save_tree};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the family tree from storage
    let mut tree = load_tree()?;
    
    // Example usage
    let person_id = tree.add_person("John Doe".to_string(), None, None, models::Gender::Male, None);
    println!("Added person with ID: {}", person_id);
    
    // Save the tree back to storage
    save_tree(&tree)?;
    
    Ok(())
} 