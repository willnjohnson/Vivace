// julian.rs
// Offsets Gregorian date by 13 days

use crate::models::CalendarDate;
use chrono::{DateTime, Local};

pub struct JulianCalendar;

impl super::Calendar for JulianCalendar {
    fn convert(&self, date: &DateTime<Local>, _settings: Option<&crate::models::UserSettings>) -> CalendarDate {
        let gregorian_date = date.naive_local().date();
        let julian_offset = 13; // Approximate offset for 21st century
        let julian_date = gregorian_date - chrono::Duration::days(julian_offset);
        CalendarDate {
            system: "Julian".to_string(),
            date: julian_date.format("%B %d, %Y").to_string(),
            additional_info: None,
        }
    }
}