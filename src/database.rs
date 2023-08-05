use crate::jobs::structs::Job;

use std::env;
use uuid::Uuid;
const DEFAULT_DB_NAME: &str = "rust-jobs";

pub struct SurrealdbConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub namespace: Option<String>,
    pub database: String,
}

impl SurrealdbConfig {
    pub fn new() -> Self {
        let port = env::var("SURREALDB_PORT").map_or_else(|e| 8080, |v| v.parse::<u16>().unwrap());
        Self {
            host: env::var("SURREALDB_HOST").expect("SURREALDB_HOST env var is mandatory!"),
            port,
            username: env::var("SURREALDB_USERNAME")
                .expect("SURREALDB_USERNAME env var is mandatory!"),
            password: env::var("SURREALDB_PASSWORD")
                .expect("SURREALDB_PASSWORD env var is mandatory!"),
            namespace: env::var("SURREALDB_NAMESPACE").ok(),
            database: env::var("SURREALDB_DATABASE")
                .map_err(|e| DEFAULT_DB_NAME)
                .unwrap(),
        }
    }
}

pub struct Db {
    pub jobs: Vec<Job>,
}

impl Db {
    pub fn new() -> Self {
        Self {
            jobs: vec![
                Job {
                    id: Uuid::new_v4().to_string(),
                    title: "Title 1".to_string(),
                },
                Job {
                    id: Uuid::new_v4().to_string(),
                    title: "Title 2".to_string(),
                },
                Job {
                    id: Uuid::new_v4().to_string(),
                    title: "Title 3".to_string(),
                },
            ],
        }
    }

    pub fn add(&mut self, job: Job) {
        self.jobs.push(job);
    }

    pub fn delete(&mut self, uuid: Uuid) {
        self.jobs.retain(|j| j.id == uuid.to_string());
    }

    pub fn get_all(&self) -> &Vec<Job> {
        &self.jobs
    }

    pub fn get_job(&self, uuid: Uuid) -> Option<&Job> {
        self.jobs.iter().find(|&j| j.id == uuid.to_string())
    }
}
