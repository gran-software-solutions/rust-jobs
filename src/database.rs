use crate::jobs::domain::{Employer, FreelanceJob, Job};

use crate::jobs::domain::{Budget, Currency, JobLocation, PermanentJob, Rate, RateTimeUnit};
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
                Job::Freelance(FreelanceJob {
                    id: Uuid::new_v4().to_string(),
                    title: "Title 1".to_string(),
                    start: Some("08.08.2023".to_string()),
                    location: JobLocation::Office,
                    employer: employers[0].to_string(),
                    description: "desc".to_string(),
                    duration_in_months: 3,
                    hours_per_week: 35,
                    office_location: Some("Address".to_string()),
                    rate: Rate::new(100, Currency::Euro, RateTimeUnit::Hour),
                }),
                Job::Freelance(FreelanceJob {
                    id: Uuid::new_v4().to_string(),
                    title: "Title 2".to_string(),
                    start: Some("02.08.2023".to_string()),
                    location: JobLocation::Office,
                    employer: employers[0].to_string(),
                    description: "desc".to_string(),
                    duration_in_months: 3,
                    hours_per_week: 35,
                    office_location: Some("Address".to_string()),
                    rate: Rate::new(100, Currency::Euro, RateTimeUnit::Hour),
                }),
                Job::Freelance(FreelanceJob {
                    id: Uuid::new_v4().to_string(),
                    title: "Title 3".to_string(),
                    start: Some("08.08.2022".to_string()),
                    location: JobLocation::Office,
                    employer: employers[0].to_string(),
                    description: "desc".to_string(),
                    duration_in_months: 3,
                    hours_per_week: 35,
                    office_location: Some("Address".to_string()),
                    rate: Rate::new(100, Currency::Euro, RateTimeUnit::Hour),
                }),
                Job::Permanent(PermanentJob {
                    id: Uuid::new_v4().to_string(),
                    title: "Title 4".to_string(),
                    start: Some("08.08.2021".to_string()),
                    location: JobLocation::Remote,
                    employer: employers[0].to_string(),
                    description: "desc".to_string(),
                    hours_per_week: 35,
                    office_location: Some("Address".to_string()),
                    budget: Budget {
                        amount: 80_000,
                        currency: Currency::Euro,
                    },
                }),
                Job::Permanent(PermanentJob {
                    id: Uuid::new_v4().to_string(),
                    title: "Title 5".to_string(),
                    start: Some("02.08.2021".to_string()),
                    location: JobLocation::Hybrid,
                    employer: employers[0].to_string(),
                    description: "desc".to_string(),
                    hours_per_week: 35,
                    office_location: Some("Address 5".to_string()),
                    budget: Budget {
                        amount: 30_000,
                        currency: Currency::Euro,
                    },
                }),
            ],
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

    pub fn add_job(&mut self, new_job: Job) {
        self.jobs.push(new_job);
    }

    pub fn add_employer<'a>(&mut self, new_employer: Employer) {
        self.employers.push(new_employer);
    }

    pub fn delete(&mut self, uuid: Uuid) {
        self.jobs.retain(|j| {
            let id = match j {
                Job::Permanent(p) => p.id,
                Job::Freelance(f) => f.id,
            };
            id != uuid.to_string()
        });
    }

    pub fn get_all(&self) -> &Vec<Job> {
        &self.jobs
    }

    pub fn get_job(&self, uuid: Uuid) -> Option<&Job> {
        self.jobs.iter().find(|j| {
            let id = match j {
                Job::Freelance(f) => f.id,
                Job::Permanent(p) => p.id,
            };
            id == uuid.to_string().as_str()
        })
    }
}
