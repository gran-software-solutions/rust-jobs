use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub enum JobType {
    Freelance,
    Permanent,
}

#[derive(Deserialize, Debug, Clone)]
pub enum Location {
    Remote,
    Office,
    Hybrid,
}

#[derive(Deserialize, Clone)]
pub struct Job {
    pub id: String,
    pub title: String,
    pub start: String,
    pub job_type: JobType,
    pub location: Location,
}

impl Job {
    pub fn new(title: String, start: String, job_type: JobType, location: Location) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            job_type,
            start,
            location,
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
