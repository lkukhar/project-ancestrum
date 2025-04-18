use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Relationship {
    Parent,
    Child,
    Sibling,
    Spouse,
} 