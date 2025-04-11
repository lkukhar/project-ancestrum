use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub birth_date: Option<String>,
    pub death_date: Option<String>,
    pub parents: Vec<String>,
    pub children: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FamilyTree {
    pub id: String,
    pub name: String,
    pub members: Vec<Person>,
} 