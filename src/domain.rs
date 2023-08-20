use std::fmt;

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub enum Role {
    HiringManager,
    Dev,
}
#[derive(Debug)]
pub enum Job {
    Permanent(PermanentJob),
    Freelance(FreelanceJob),
}

impl fmt::Display for Job {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct User {
    pub email: String,
    pub password: String,
    pub roles: Vec<Role>,
}

#[derive(Debug)]
pub struct Address {
    pub city: String,
    pub country: String,
}

#[derive(Debug)]
pub struct Employer {
    pub name: String,
    pub address: Address,
}

#[derive(Debug)]
pub enum FreelanceRateUnit {
    Hour,
    Day,
}

#[derive(Debug)]
pub struct FreelanceRate {
    pub amount: f64,
    pub currency: Currency,
    pub unit: FreelanceRateUnit,
}

#[derive(Debug)]
pub enum Currency {
    EUR,
    USD,
}

#[derive(Debug)]
pub struct Salary {
    pub amount: f64,
    pub currency: Currency,
}

#[derive(Debug)]
pub enum JobLocation {
    OnSite,
    Hybrid,
    Remote,
}

impl fmt::Display for JobLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl fmt::Display for FreelanceRateUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct FreelanceJob {
    pub id: Uuid,
    pub title: String,
    pub location: JobLocation,
    pub rate: FreelanceRate,
    pub start: DateTime<Utc>,
    pub employer: Employer,
    pub requires_insurance: bool,
    pub last_updated_on: DateTime<Utc>,
}

#[derive(Debug)]
pub struct PermanentJob {
    pub id: Uuid,
    pub title: String,
    pub location: JobLocation,
    pub salary: Salary,
    pub start: DateTime<Utc>,
    pub employer: Employer,
    pub last_updated_on: DateTime<Utc>,
}
