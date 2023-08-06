use serde::Deserialize;

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
    pub client: String,
}

#[derive(Deserialize)]
pub struct Employer {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}
