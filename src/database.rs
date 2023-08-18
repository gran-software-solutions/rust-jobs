use chrono::{TimeZone, Utc};

use crate::domain::{
    Address, Currency, Employer, FreelanceJob, FreelanceRate, FreelanceRateUnit, Job, JobLocation,
    PermanentJob, Role, Salary, User,
};

pub struct Database {
    users: Vec<User>,
    jobs: Vec<Job>,
}

impl Database {
    pub fn get_jobs(&self, take: Option<usize>) -> Vec<&Job> {
        let jobs = match take {
            Some(how_many) => self.jobs.iter().take(how_many).collect(),
            None => self.jobs.iter().collect(),
        };
        jobs
    }
    pub fn new() -> Database {
        Database {
            users: vec![
                User {
                    email: "gg@gransoftware.de".to_string(),
                    roles: vec![Role::HiringManager, Role::Dev],
                },
                User {
                    email: "do@gransoftware.de".to_string(),
                    roles: vec![Role::HiringManager, Role::Dev],
                },
                User {
                    email: "rust@dev.de".to_string(),
                    roles: vec![Role::Dev],
                },
                User {
                    email: "hiring@manager.de".to_string(),
                    roles: vec![Role::HiringManager],
                },
            ],
            jobs: vec![
                Job::Freelance(FreelanceJob {
                    title: "Senior Rust Dev".to_string(),
                    start: Utc::now(),
                    requires_insurance: false,
                    location: JobLocation::Remote,
                    rate: FreelanceRate {
                        amount: 800.0,
                        unit: FreelanceRateUnit::Day,
                        currency: Currency::EUR,
                    },
                    employer: Employer {
                        name: "GRAN Software Solutions GmbH".to_string(),
                        address: Address {
                            city: "Berlin".to_string(),
                            country: "Gernany".to_string(),
                        },
                    },
                    last_updated_on: Utc::now(),
                }),
                Job::Freelance(FreelanceJob {
                    title: "Senior Rust Dev urgent".to_string(),
                    start: Utc::now(),
                    requires_insurance: false,
                    location: JobLocation::Remote,
                    rate: FreelanceRate {
                        amount: 80.0,
                        unit: FreelanceRateUnit::Hour,
                        currency: Currency::EUR,
                    },
                    employer: Employer {
                        name: "GRAN Software Solutions GmbH".to_string(),
                        address: Address {
                            city: "Berlin".to_string(),
                            country: "Gernany".to_string(),
                        },
                    },
                    last_updated_on: Utc::now(),
                }),
                Job::Permanent(PermanentJob {
                    title: "Permanent Senior Rust Dev".to_string(),
                    start: Utc::now(),
                    location: JobLocation::Hybrid,
                    salary: Salary {
                        amount: 80000.0,
                        currency: Currency::EUR,
                    },
                    employer: Employer {
                        name: "GRAN Software Solutions GmbH".to_string(),
                        address: Address {
                            city: "Berlin".to_string(),
                            country: "Germany".to_string(),
                        },
                    },
                    last_updated_on: Utc::now(),
                }),
                Job::Permanent(PermanentJob {
                    title: "Junior Senior Rust Dev".to_string(),
                    start: Utc.with_ymd_and_hms(2023, 8, 12, 0, 0, 0).unwrap(),
                    location: JobLocation::OnSite,
                    salary: Salary {
                        amount: 40000.0,
                        currency: Currency::USD,
                    },
                    employer: Employer {
                        name: "Acme LLC".to_string(),
                        address: Address {
                            city: "San Diego".to_string(),
                            country: "California".to_string(),
                        },
                    },
                    last_updated_on: Utc::now(),
                }),
            ],
        }
    }
}
