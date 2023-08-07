use super::jobs::domain::Job;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "home.stpl")]
pub struct Home<'a> {
    pub title: &'a str,
    pub jobs: &'a Vec<Job>,
}

pub fn home_presenter(jobs: &Vec<Job>) -> String {
    Home {
        jobs,
        title: "Homepage",
    }
    .render_once()
    .unwrap()
}

#[derive(TemplateOnce)]
#[template(path = "job.stpl")]
pub struct JobDetails<'a> {
    pub title: &'a str,
    pub job: &'a Job,
}

#[derive(TemplateOnce)]
#[template(path = "notfound.stpl")]
pub struct NotFound<'a> {
    pub message: &'a str,
}

pub fn job_details_presenter(job: &Job) -> String {
    JobDetails {
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
#[template(path = "new_job.stpl")]
pub struct NewJobView;

pub fn new_job_presenter() -> String {
    NewJobView {}.render_once().unwrap()
}
