use crate::jobs::{
    self,
    domain::{Employer, FreelanceJob},
};

use crate::jobs::domain::RegularJob;
use uuid::Uuid;

pub struct Db {
    pub freelance_jobs: Vec<FreelanceJob>,
    pub regular_jobs: Vec<RegularJob>,
    pub employers: Vec<Employer>,
}

impl Db {
    pub fn new() -> Self {
        let employers = ["Sony", "Google", "Amazon"];
        Self {
            freelance_jobs: vec![
                FreelanceJob {
                    id: Uuid::new_v4().to_string(),
                    title: "Title 1".to_string(),
                    start: String::from("asap"),
                    job_type: jobs::domain::JobType::Permanent,
                    location: jobs::domain::Location::Remote,
                    employer: employers[0].to_string(),
                },
                FreelanceJob {
                    id: Uuid::new_v4().to_string(),
                    title: "Title 2".to_string(),
                    start: String::from("asap"),
                    job_type: jobs::domain::JobType::Freelance,
                    location: jobs::domain::Location::Office,
                    employer: employers[1].to_string(),
                },
                FreelanceJob {
                    id: Uuid::new_v4().to_string(),
                    title: "Title 3".to_string(),
                    start: String::from("asap"),
                    job_type: jobs::domain::JobType::Permanent,
                    location: jobs::domain::Location::Hybrid,
                    employer: employers[2].to_string(),
                },
            ],
            regular_jobs: vec![RegularJob {}],
            employers: vec![
                Employer {
                    id: Uuid::new_v4().to_string(),
                    name: "Microsoft".to_string(),
                    email: "yolo@gransoftware.de".to_string(),
                    password: "secret123".to_string(),
                },
                Employer {
                    id: Uuid::new_v4().to_string(),
                    name: "Google".to_string(),
                    email: "goo@gle.de".to_string(),
                    password: "secret123".to_string(),
                },
            ],
        }
    }

    pub fn add_job(&mut self, new_job: FreelanceJob) {
        self.freelance_jobs.push(new_job);
    }

    pub fn add_employer<'a>(&mut self, new_employer: Employer) {
        self.employers.push(new_employer);
    }

    pub fn delete(&mut self, uuid: Uuid) {
        self.freelance_jobs.retain(|j| j.id != uuid.to_string());
    }

    pub fn get_all(&self) -> &Vec<FreelanceJob> {
        &self.freelance_jobs
    }

    pub fn get_freelance_job(&self, uuid: Uuid) -> Option<&FreelanceJob> {
        self.freelance_jobs
            .iter()
            .find(|&j| j.id == uuid.to_string())
    }

    pub fn get_regular_job(&self, uuid: Uuid) -> Option<&RegularJob> {
        self.regular_jobs.iter().find(|&j| j.id == uuid.to_string())
    }
}
