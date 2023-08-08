use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub enum JobType {
    Freelance,
    Permanent,
}

#[derive(Deserialize, Clone, Debug)]
pub enum JobLocation {
    Office,
    Remote,
    Hybrid,
}

#[derive(Deserialize, Clone)]
pub enum RateCurrency {
    Euro,
    Dollar,
}

#[derive(Deserialize, Clone)]
pub enum RateTimeUnit {
    Hour,
    Day,
}

#[derive(Deserialize, Clone)]
pub struct Rate {
    pub amount: u16,
    pub currency: RateCurrency,
    pub time_unit: RateTimeUnit,
}

impl Rate {
    pub fn new(amount: u16, currency: RateCurrency, time_unit: RateTimeUnit) -> Self {
        Self {
            amount,
            currency,
            time_unit,
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct FreelanceJob {
    pub id: String,
    pub employer: String,
    pub title: String,
    pub start: Option<String>,
    pub duration_in_months: u16,
    pub rate: Rate,
    pub hours_per_week: u8,
    pub location: JobLocation,
    pub office_location: Option<String>,
    pub description: String,
}

#[derive(Deserialize)]
pub struct Employer {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}
