use maud::{html, Markup, DOCTYPE};

use crate::session_state::TypedSession;

pub fn header(is_logged_in: bool) -> Markup {
    html! {
        header class="header" {
            div class="pure-g" {
                div class="pure-u-1-3 rust-jobs" {
                    img src="https://img.linuxfr.org/img/687474703a2f2f7777772e727573742d6c616e672e6f72672f6c6f676f732f727573742d6c6f676f2d323536783235362d626c6b2e706e67/rust-logo-256x256-blk.png" alt="Rust Logo" class="rust-logo";
                    a href="/" {
                        h1 class="logo" {
                            "Rust Jobs"
                        }
                    }
                }
                div class="pure-u-2-3 header-links" {
                    ul class="pure-menu pure-menu-horizontal pure-menu-right" {
                        @if is_logged_in {
                            li class="pure-menu-item" {
                                a href="/signout" class="pure-menu-link" {
                                    "Sign Out"
                                }
                            }
                        } @else {
                            li class="pure-menu-item" {
                                a href="#" class="pure-menu-link" {
                                    "Sign Up"
                                }
                            }
                            li class="pure-menu-item" {
                                a href="/signin" class="pure-menu-link" {
                                    "Sign In"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn head(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        meta viewport="width=device-width, initial-scale=1.0";
        link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/pure/2.0.3/pure-min.css";
        link rel="stylesheet" href="/static/css/styles.css";
        link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Alfa+Slab+One&family=Fira+Sans:wght@400;700&display=swap";
        title { (page_title) }
    }
}

pub fn footer() -> Markup {
    html! {
        div class="footer" {
            div class="footer-column" {
                div class="footer-column-title" { "Terms and policies" }
                a href="#" { "Random Term" }
                a href="#" { "Random Policy" }
                a href="#" { "Random Agreement" }
            }
            div class="footer-column" {
                div class="footer-column-title" { "Social" }
                a href="#" { "Random Term" }
                a href="#" { "Random Policy" }
                a href="#" { "Random Agreement" }
            }
        }
    }
}

pub fn not_found(msg: &str, session: TypedSession) -> Markup {
    html! {
        (head("Oooops! 404"))
        (header(session.get_user_id().unwrap().is_some()))
            h1 { (msg)}
            div class="centered-link" {
                a href="/" {
                    "To homepage"
                }
            }
            (footer())
    }
}
