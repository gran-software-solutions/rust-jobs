use crate::jobs::structs::JobDetails;

use super::jobs::structs::{Home, Job};
use sailfish::TemplateOnce;

pub fn home_presenter(jobs: &Vec<Job>) -> String {
    Home {
        jobs,
        title: "Homepage",
    }
    .render_once()
    .unwrap()
}

pub fn job_details_presenter(job: &Job) -> String {
    JobDetails {
        title: &job.title[..],
        job: job,
    }
    .render_once()
    .unwrap()
}
