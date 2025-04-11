use uuid::Uuid;

pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn validate_date(date: &str) -> bool {
    // TODO: Implement proper date validation
    !date.is_empty()
} 