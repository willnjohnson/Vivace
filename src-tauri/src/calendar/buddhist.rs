// buddhist.rs
// Adds 543 to the standard year

use crate::models::CalendarDate;
use chrono::{DateTime, Local, Datelike};

pub struct BuddhistCalendar;

impl super::Calendar for BuddhistCalendar {
    fn convert(&self, date: &DateTime<Local>, _settings: Option<&crate::models::UserSettings>) -> CalendarDate {
        let buddhist_year = date.year() + 543;
        CalendarDate {
            system: "Buddhist".to_string(),
            date: format!("{}, {} BE", date.format("%B %d"), buddhist_year),
            additional_info: None,
        }
    }
}