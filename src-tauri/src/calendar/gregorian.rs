// gregorian.rs
// Standard calendar

use crate::models::CalendarDate;
use crate::utils::resolve_date_format;
use chrono::{DateTime, Local};

pub struct GregorianCalendar;

impl super::Calendar for GregorianCalendar {
    fn convert(&self, date: &DateTime<Local>, settings: Option<&crate::models::UserSettings>) -> CalendarDate {
        let format_str = settings
            .map(|s| resolve_date_format(s.date_format.as_deref(), s.show_seconds.unwrap_or(false)))
            .unwrap_or_else(|| "%A, %B %d, %Y".to_string());

        CalendarDate {
            system: "Gregorian".to_string(),
            date: date.format(&format_str).to_string(),
            additional_info: None,
        }
    }
}