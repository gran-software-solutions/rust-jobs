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

#[derive(Deserialize)]
pub struct Employer {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}
