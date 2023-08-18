use actix_web::web;
use maud::{html, Markup};
use uuid::Uuid;

use crate::{
    database::Database,
    domain::{FreelanceJob, Job, PermanentJob},
    handlers::{footer, head, header},
};

use super::not_found;

pub struct SearchResultJob {
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

fn render_freelance_job(freelance_job: &FreelanceJob) -> Markup {
    html! {
        (head("Freelance Job details"))
        (header())
        h1 { "Freelance Job: " (freelance_job.title) }
        table class="pure-table pure-table-horizontal centered-table" {
            tr {
                th class="highlighted-cell" {
                    "Id"
                }
                th {
                    (freelance_job.id)
                }
            }
            tr {
                th class="highlighted-cell" {
                    "Title"
                }
                th {
                    (freelance_job.title)
                }
            }
            tr {
                th class="highlighted-cell" {
                    "Employer name"
                }
                th {
                    (freelance_job.employer.name)
                }
            }
            tr {
                th class="highlighted-cell" {
                    "Employer location"
                }
                th {
                    (freelance_job.employer.address.city) " (" (freelance_job.employer.address.country) ")"
                }
            }
            tr {
                th class="highlighted-cell" {
                    "Location"
                }
                th {
                    (freelance_job.location.to_string())
                }
            }
            tr {
                th class="highlighted-cell" {
                    "Budget"
                }
                th {
                    (freelance_job.rate.amount) " " (freelance_job.rate.currency) " / " (freelance_job.rate.unit)
                }
            }
            tr {
                th class="highlighted-cell" {
                    "Start"
                }
                th {
                    (freelance_job.start)
                }
            }
        }
        (footer())
    }
}
fn render_permanent_job(permanent_job: &PermanentJob) -> Markup {
    html! {
        (head("Permanent Job details"))
        (header())
        h1 { "Permanent Job: " (permanent_job.title) }
        table class="pure-table pure-table-horizontal centered-table" {
            tr {
                th class="highlighted-cell" {
                    "Id"
                }
                th {
                    (permanent_job.id)
                }
            }
            tr {
                th class="highlighted-cell" {
                    "Title"
                }
                th {
                    (permanent_job.title)
                }
            }
            tr {
                th class="highlighted-cell" {
                    "Employer name"
                }
                th {
                    (permanent_job.employer.name)
                }
            }
            tr {
                th class="highlighted-cell" {
                    "Employer location"
                }
                th {
                    (permanent_job.employer.address.city) " (" (permanent_job.employer.address.country) ")"
                }
            }
            tr {
                th class="highlighted-cell" {
                    "Location"
                }
                th {
                    (permanent_job.location.to_string())
                }
            }
            tr {
                th class="highlighted-cell" {
                    "Budget"
                }
                th {
                    (permanent_job.salary.amount) " " (permanent_job.salary.currency) " / annum"
                }
            }
            tr {
                th class="highlighted-cell" {
                    "Start"
                }
                th {
                    (permanent_job.start)
                }
            }
        }
        (footer())
    }
}

pub async fn job_details(
    db: web::Data<Database>,
    id: web::Path<Uuid>,
) -> actix_web::Result<Markup> {
    let job_id: Uuid = id.into_inner();
    match db.get_job(job_id) {
        Some(Job::Freelance(f)) => Ok(render_freelance_job(f)),
        Some(Job::Permanent(p)) => Ok(render_permanent_job(p)),
        None => Ok(not_found("We couldn't find such job! ")),
    }
}
