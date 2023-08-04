use sailfish::TemplateOnce;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Job {
    pub id: String,
    pub title: String,
}

#[derive(TemplateOnce)]
#[template(path = "home.stpl")]
pub struct Home<'a> {
    pub title: &'a str,
    pub jobs: &'a Vec<Job>,
}

#[derive(TemplateOnce)]
#[template(path = "job.stpl")]
pub struct JobDetails<'a> {
    pub title: &'a str,
    pub job: &'a Job,
}
