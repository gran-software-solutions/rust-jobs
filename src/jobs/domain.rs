use std::fmt;

use sailfish::runtime::Render;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub enum JobType {
    Freelance,
    Permanent,
}

impl fmt::Display for JobType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Render for JobType {
    fn render(&self, b: &mut sailfish::runtime::Buffer) -> Result<(), sailfish::RenderError> {
        b.push_str(self.to_string().as_str());
        Ok(())
    }
}

#[derive(Deserialize, Clone)]
pub struct Job {
    pub id: String,
    pub title: String,
    pub start: String,
    pub job_type: JobType,
}

impl Job {
    pub fn new(title: String, start: String, job_type: JobType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            job_type,
            start,
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
