use maud::{html, Markup};

use crate::handlers::{footer, head, header};

pub struct HomepageJob {
    pub title: String,
    pub job_type: String,
    pub mode: String,
    pub place: String,
    pub budget: String,
}

pub async fn homepage() -> actix_web::Result<Markup> {
    let jobs = vec![
        HomepageJob {
            title: "Senior Rust Developer".to_string(),
            job_type: "Freelance".to_string(),
            mode: "On-site".to_string(),
            place: "Berlin (Germany)".to_string(),
            budget: "90€/h".to_string(),
        },
        HomepageJob {
            title: "Rust Dev".to_string(),
            job_type: "Permanent".to_string(),
            mode: "Hybrid".to_string(),
            place: "Ingolstadt (Germany)".to_string(),
            budget: "90000 €/annum".to_string(),
        },
        HomepageJob {
            title: "Lead Rust Developer".to_string(),
            job_type: "Freelance".to_string(),
            mode: "Remote".to_string(),
            place: "Berlin (Germany)".to_string(),
            budget: "90€/h".to_string(),
        },
    ];
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
                        }
                    }
                    tbody {
                        @for job in &jobs {
                            tr {
                                td class="fira-sans" {
                                    (job.title)
                                }
                                td class="fira-sans" {
                                    (job.job_type)
                                }
                                td class="fira-sans" {
                                    (job.mode)
                                }
                                td class="fira-sans" {
                                    (job.place)
                                }
                                td class="fira-sans" {
                                    (job.budget)
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
