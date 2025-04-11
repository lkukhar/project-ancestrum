use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDate;
use crate::models::gender::Gender;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub birth_date: Option<NaiveDate>,
    pub death_date: Option<NaiveDate>,
    pub gender: Gender,
    pub notes: String,
} 