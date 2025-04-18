use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Other,
} 