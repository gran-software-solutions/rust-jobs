use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Clone)]
pub struct Job {
    pub id: String,
    pub title: String,
}

impl Job {
    pub fn new(title: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
        }
    }
}
