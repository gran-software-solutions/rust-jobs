use crate::jobs::{
    self,
    domain::{Employer, Job},
};

use uuid::Uuid;

pub struct Db {
    pub jobs: Vec<Job>,
    pub employers: Vec<Employer>,
}

impl Db {
    pub fn new() -> Self {
        let employers = ["Sony", "Google", "Amazon"];
        Self {
            jobs: vec![
                Job {
                    id: Uuid::new_v4().to_string(),
                    title: "Title 1".to_string(),
                    start: String::from("asap"),
                    job_type: jobs::domain::JobType::Permanent,
                    location: jobs::domain::Location::Remote,
                    employer: employers[0].to_string(),
                },
                Job {
                    id: Uuid::new_v4().to_string(),
                    title: "Title 2".to_string(),
                    start: String::from("asap"),
                    job_type: jobs::domain::JobType::Freelance,
                    location: jobs::domain::Location::Office,
                    employer: employers[1].to_string(),
                },
                Job {
                    id: Uuid::new_v4().to_string(),
                    title: "Title 3".to_string(),
                    start: String::from("asap"),
                    job_type: jobs::domain::JobType::Permanent,
                    location: jobs::domain::Location::Hybrid,
                    employer: employers[2].to_string(),
                },
            ],
            employers: vec![Employer {
                id: Uuid::new_v4().to_string(),
                name: "Microsfot".to_string(),
                email: "yolo@gransoftware.de".to_string(),
                password: "secret123".to_string(),
            }],
        }
    }

    pub fn add_job(&mut self, new_job: Job) {
        self.jobs.push(new_job);
    }

    pub fn add_employer<'a>(&mut self, new_employer: Employer) {
        self.employers.push(new_employer);
    }

    pub fn delete(&mut self, uuid: Uuid) {
        self.jobs.retain(|j| j.id != uuid.to_string());
    }

    pub fn get_all(&self) -> &Vec<Job> {
        &self.jobs
    }

    pub fn get_job(&self, uuid: Uuid) -> Option<&Job> {
        self.jobs.iter().find(|&j| j.id == uuid.to_string())
    }
}
