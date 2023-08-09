use sailfish::TemplateOnce;

use crate::jobs::domain::{Job, JobType, PermanentJob};

use super::jobs::domain::FreelanceJob;

pub trait HomepageJob {
    fn get_title(&self) -> &str;
    fn get_id(&self) -> &str;
    fn get_job_type(&self) -> &JobType;
}

impl HomepageJob for Job {
    fn get_title(&self) -> &str {
        match self {
            Job::Permanent(p) => p.title.as_str(),
            Job::Freelance(f) => f.title.as_str(),
        }
    }
    fn get_id(&self) -> &str {
        match self {
            Job::Permanent(p) => p.id.as_str(),
            Job::Freelance(f) => f.id.as_str(),
        }
    }
    fn get_job_type(&self) -> &JobType {
        &match self {
            Job::Permanent(_) => JobType::Permanent,
            Job::Freelance(_) => JobType::Freelance,
        }
    }
}

#[derive(TemplateOnce)]
#[template(path = "home.stpl")]
pub struct Home<T: HomepageJob> {
    pub jobs: Vec<T>,
}

pub fn home_presenter(jobs: Vec<impl HomepageJob>) -> String {
    Home { jobs }.render_once().unwrap()
}

#[derive(TemplateOnce)]
#[template(path = "job/freelance_job.stpl")]
pub struct FreelanceJobDetails<'a> {
    pub job: &'a FreelanceJob,
}

#[derive(TemplateOnce)]
#[template(path = "job/permanent_job.stpl")]
pub struct PermanentJobDetails<'a> {
    pub job: &'a PermanentJob,
}

#[derive(TemplateOnce)]
#[template(path = "notfound.stpl")]
pub struct NotFound<'a> {
    pub message: &'a str,
}

pub fn permanent_job_presenter(job: &PermanentJob) -> String {
    PermanentJobDetails { job }.render_once().unwrap()
}

pub fn freelance_job_presenter(job: &FreelanceJob) -> String {
    FreelanceJobDetails { job }.render_once().unwrap()
}

pub fn not_found(message: String) -> String {
    NotFound { message: &message }.render_once().unwrap()
}

#[derive(TemplateOnce)]
#[template(path = "job/new_freelance_job.stpl")]
pub struct NewPermanentJobView;

pub fn new_permanent_job_presenter() -> String {
    NewPermanentJobView {}.render_once().unwrap()
}

#[derive(TemplateOnce)]
#[template(path = "job/new_freelance_job.stpl")]
pub struct NewFreelanceJobView;

pub fn new_freelance_job_presenter() -> String {
    NewFreelanceJobView {}.render_once().unwrap()
}
