use actix_web::web;
use maud::{html, Markup};

use crate::{
    database::Database,
    domain::{FreelanceJob, Job, PermanentJob},
    handlers::{footer, head, header},
};

pub struct HomepageJob {
    pub title: String,
    pub job_type: String,
    pub location: String,
    pub employer_name: String,
    pub address: String,
    pub budget: String,
    pub last_updated_on: String,
}

impl From<&FreelanceJob> for HomepageJob {
    fn from(value: &FreelanceJob) -> Self {
        HomepageJob {
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
impl From<&PermanentJob> for HomepageJob {
    fn from(value: &PermanentJob) -> HomepageJob {
        HomepageJob {
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

impl From<&Job> for HomepageJob {
    fn from(value: &Job) -> Self {
        match value {
            Job::Freelance(fj) => fj.into(),
            Job::Permanent(p) => p.into(),
        }
    }
}

pub async fn homepage(db: web::Data<Database>) -> actix_web::Result<Markup> {
    let jobs: Vec<HomepageJob> = db
        .get_jobs(Some(5))
        .iter()
        .map(|j| HomepageJob::from(*j))
        .collect();
    Ok(html! {
        (head("Homepage"))
        (header())
        div class="content-container" {
            div class="content" {
                h1 class="centered-text job-count-text" {
                    "Currently there are " span class="job-count" { "300" } " rust jobs listed"
                }
                table class="pure-table centered-table" {
                    thead {
                        tr class="table-header" {
                            th { "Job Title" }
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
                                    (job.title)
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
                div class="centered-link" {
                    a href="#" class="normal-link" {
                        "See the full list"
                    }
                }
                div class="container" {
                    div class="box" {
                        p {
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod quam eu mauris laoreet, sit
                            amet dictum urna feugiat."
                        }
                        p {
                            "Integer scelerisque libero sit amet ligula sagittis, nec laoreet elit fermentum."
                        }
                        a href="/signup/employer" class="register-button" {
                            "Post A Job Today As Employer"
                        }
                    }
                    div class="box" {
                        p {
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod quam eu mauris laoreet, sit
                            amet dictum urna feugiat."
                        }
                        p {
                            "Integer scelerisque libero sit amet ligula sagittis, nec laoreet elit fermentum."
                        }
                        a href="/signup/dev" class="register-button" {
                            "Register As A Rust Developer"
                        }
                    }
                }
            }
        }
        (footer())
    })
}
