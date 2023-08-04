use crate::Job;
use uuid::Uuid;
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
