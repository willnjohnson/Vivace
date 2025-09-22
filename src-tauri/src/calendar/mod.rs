// mod.rs
// Maintains model

pub mod gregorian;
pub mod julian;
pub mod buddhist;
pub mod french_revolutionary;
pub mod jewish;

use crate::models::CalendarDate;
use chrono::{DateTime, Local};

pub trait Calendar {
    fn convert(&self, date: &DateTime<Local>, settings: Option<&crate::models::UserSettings>) -> CalendarDate;
}