use crate::domain::{
    Address, Currency, Employer, FreelanceJob, FreelanceRate, FreelanceRateUnit, Job, JobLocation,
    PermanentJob, Role, Salary, User,
};
use chrono::{TimeZone, Utc};
use uuid::Uuid;

pub struct Database {
    users: Vec<User>,
    jobs: Vec<Job>,
}

impl Database {
    pub fn get_job(&self, job_id: Uuid) -> Option<&Job> {
        self.jobs.iter().find(|&j| match j {
            Job::Freelance(f) => f.id == job_id,
            Job::Permanent(p) => p.id == job_id,
        })
    }
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
                    id: Uuid::new_v4(),
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
                    id: Uuid::new_v4(),
                    title: "Rust Engiener".to_string(),
                    start: Utc::now(),
                    requires_insurance: true,
                    location: JobLocation::Hybrid,
                    rate: FreelanceRate {
                        amount: 120.0,
                        unit: FreelanceRateUnit::Hour,
                        currency: Currency::EUR,
                    },
                    employer: Employer {
                        name: "Bettermile AG".to_string(),
                        address: Address {
                            city: "Berlin".to_string(),
                            country: "Gernany".to_string(),
                        },
                    },
                    last_updated_on: Utc::now(),
                }),
                Job::Freelance(FreelanceJob {
                    id: Uuid::new_v4(),
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
                Job::Freelance(FreelanceJob {
                    id: Uuid::new_v4(),
                    title: "Apprentice Rust Dev sofort".to_string(),
                    start: Utc::now(),
                    requires_insurance: true,
                    location: JobLocation::Hybrid,
                    rate: FreelanceRate {
                        amount: 20.0,
                        unit: FreelanceRateUnit::Hour,
                        currency: Currency::EUR,
                    },
                    employer: Employer {
                        name: "VW AG".to_string(),
                        address: Address {
                            city: "München".to_string(),
                            country: "Gernany".to_string(),
                        },
                    },
                    last_updated_on: Utc::now(),
                }),
                Job::Permanent(PermanentJob {
                    id: Uuid::new_v4(),
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
                    id: Uuid::new_v4(),
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
                Job::Permanent(PermanentJob {
                    id: Uuid::new_v4(),
                    title: "Medior Senior Rust Dev".to_string(),
                    start: Utc.with_ymd_and_hms(2023, 3, 22, 0, 0, 0).unwrap(),
                    location: JobLocation::Hybrid,
                    salary: Salary {
                        amount: 33000.0,
                        currency: Currency::EUR,
                    },
                    employer: Employer {
                        name: "Acme 2 LLC".to_string(),
                        address: Address {
                            city: "San Francisco".to_string(),
                            country: "California".to_string(),
                        },
                    },
                    last_updated_on: Utc::now(),
                }),
                Job::Permanent(PermanentJob {
                    id: Uuid::new_v4(),
                    title: "Rust guru".to_string(),
                    start: Utc.with_ymd_and_hms(2023, 9, 12, 0, 0, 0).unwrap(),
                    location: JobLocation::OnSite,
                    salary: Salary {
                        amount: 12000.0,
                        currency: Currency::USD,
                    },
                    employer: Employer {
                        name: "Acme 5 LLC".to_string(),
                        address: Address {
                            city: "Sacramento".to_string(),
                            country: "California".to_string(),
                        },
                    },
                    last_updated_on: Utc::now(),
                }),
                Job::Permanent(PermanentJob {
                    id: Uuid::new_v4(),
                    title: "Rust lead".to_string(),
                    start: Utc.with_ymd_and_hms(2023, 3, 11, 0, 0, 0).unwrap(),
                    location: JobLocation::OnSite,
                    salary: Salary {
                        amount: 33000.0,
                        currency: Currency::USD,
                    },
                    employer: Employer {
                        name: "Acme 3 LLC".to_string(),
                        address: Address {
                            city: "Las Vegas".to_string(),
                            country: "Utah".to_string(),
                        },
                    },
                    last_updated_on: Utc::now(),
                }),
            ],
        }
    }
}
