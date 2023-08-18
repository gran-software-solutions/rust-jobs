use actix_web::web;
use maud::{html, Markup};
use uuid::Uuid;

use crate::{
    database::Database,
    domain::{FreelanceJob, Job, PermanentJob},
    handlers::{footer, head, header},
};

pub struct SearchResultJob {
    pub id: Uuid,
    pub title: String,
    pub job_type: String,
    pub location: String,
    pub employer_name: String,
    pub address: String,
    pub budget: String,
    pub last_updated_on: String,
}

impl From<&FreelanceJob> for SearchResultJob {
    fn from(value: &FreelanceJob) -> Self {
        SearchResultJob {
            id: value.id,
            title: value.title.clone(),
            job_type: "Freelance".to_string(),
            location: value.location.to_string(),
            employer_name: value.employer.name.clone(),
            address: format!(
                "{} ({})",
                value.employer.address.city, value.employer.address.country
            ),
            budget: format!(
                "{} {} / {}",
                value.rate.amount, value.rate.currency, value.rate.unit,
            ),
            last_updated_on: value
                .last_updated_on
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        }
    }
}
impl From<&PermanentJob> for SearchResultJob {
    fn from(value: &PermanentJob) -> SearchResultJob {
        SearchResultJob {
            id: value.id,
            title: value.title.clone(),
            job_type: "Permanent".to_string(),
            location: value.location.to_string(),
            employer_name: value.employer.name.clone(),
            budget: format!("{} {} / annum", value.salary.amount, value.salary.currency),
            address: format!(
                "{} ({})",
                value.employer.address.city, value.employer.address.country
            ),
            last_updated_on: value
                .last_updated_on
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        }
    }
}

impl From<&Job> for SearchResultJob {
    fn from(value: &Job) -> Self {
        match value {
            Job::Freelance(fj) => fj.into(),
            Job::Permanent(p) => p.into(),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct JobSearch {}

pub async fn job_search(
    db: web::Data<Database>,
    _search: web::Query<JobSearch>,
) -> actix_web::Result<Markup> {
    let jobs: Vec<SearchResultJob> = db
        .get_jobs(None)
        .iter()
        .map(|j| SearchResultJob::from(*j))
        .collect();
    Ok(html! {
        (head("Search Jobs Page"))
        (header())
        div class="content-container" {
            div class="content" {
                h1 class="centered-text job-count-text" {
                    "We have " span class="job-count" { (jobs.len()) } " rust jobs for You"
                }
                table class="pure-table centered-table" {
                    thead {
                        tr class="table-header" {
                            th { "Job Title" }
                            th { "Employer" }
                            th { "Perm or Freelance" }
                            th { "Type" }
                            th { "Location" }
                            th { "Budget" }
                            th { "Updated on" }
                        }
                    }
                    tbody {
                        @for job in jobs {
                            tr {
                                td class="fira-sans" {
                                    a href={"/jobs/" (job.id)} {
                                        (job.title)
                                    }
                                }
                                td class="fira-sans" {
                                    (job.employer_name)
                                }
                                td class="fira-sans" {
                                    (job.job_type)
                                }
                                td class="fira-sans" {
                                    (job.location)
                                }
                                td class="fira-sans" {
                                    (job.location)
                                }
                                td class="fira-sans" {
                                    (job.budget)
                                }
                                td class="fira-sans" {
                                    (job.last_updated_on)
                                }
                            }
                        }
                    }
                }
            }
        }
        (footer())
    })
}
