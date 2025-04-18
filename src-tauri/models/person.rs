use serde::{Deserialize, Serialize};
use super::Gender;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub birth_date: Option<String>,
    pub death_date: Option<String>,
    pub gender: Gender,
    pub notes: String,
} 