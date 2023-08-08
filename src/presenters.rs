use super::jobs::domain::FreelanceJob;
use crate::jobs::domain::RegularJob;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "home.stpl")]
pub struct Home<'a> {
    pub title: &'a str,
    pub jobs: &'a Vec<FreelanceJob>,
}

pub fn home_presenter(jobs: &Vec<FreelanceJob>) -> String {
    Home {
        jobs,
        title: "Homepage",
    }
    .render_once()
    .unwrap()
}

#[derive(TemplateOnce)]
#[template(path = "job/freelance_job.stpl")]
pub struct FreelanceJobDetails<'a> {
    pub title: &'a str,
    pub job: &'a FreelanceJob,
}

#[derive(TemplateOnce)]
#[template(path = "job/regular_job.stpl")]
pub struct RegularJobDetails<'a> {
    pub title: &'a str,
    pub job: &'a RegularJob,
}

#[derive(TemplateOnce)]
#[template(path = "notfound.stpl")]
pub struct NotFound<'a> {
    pub message: &'a str,
}

pub fn regular_job_presenter(job: &RegularJob) -> String {
    RegularJobDetails {
        title: &job.title[..],
        job,
    }
    .render_once()
    .unwrap()
}

pub fn freelance_job_presenter(job: &FreelanceJob) -> String {
    FreelanceJobDetails {
        title: &job.title[..],
        job,
    }
    .render_once()
    .unwrap()
}

pub fn not_found(message: String) -> String {
    NotFound { message: &message }.render_once().unwrap()
}

#[derive(TemplateOnce)]
#[template(path = "job/new_freelance_job.stpl")]
pub struct NewRegularJobView;

pub fn new_regular_job_presenter() -> String {
    NewRegularJobView {}.render_once().unwrap()
}

#[derive(TemplateOnce)]
#[template(path = "job/new_freelance_job.stpl")]
pub struct NewFreelanceJobView;

pub fn new_freelance_job_presenter() -> String {
    NewFreelanceJobView {}.render_once().unwrap()
}
