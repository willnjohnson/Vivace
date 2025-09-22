// calendar_commands.rs
// Maintains calendar commands

use crate::calendar::{
    buddhist::BuddhistCalendar,
    french_revolutionary::FrenchRevolutionaryCalendar,
    gregorian::GregorianCalendar,
    jewish::JewishCalendar,
    julian::JulianCalendar,
};
use crate::models::CalendarDate;
use crate::settings::load_settings;
use std::collections::HashMap;

#[tauri::command]
pub fn get_current_dates() -> Result<Vec<CalendarDate>, String> {
    let settings = load_settings()?;
    let now = chrono::Local::now();
    let mut dates = Vec::new();

    let calendars: HashMap<&str, Box<dyn crate::calendar::Calendar>> = [
        ("gregorian", Box::new(GregorianCalendar) as Box<dyn crate::calendar::Calendar>),
        ("julian", Box::new(JulianCalendar)),
        ("buddhist", Box::new(BuddhistCalendar)),
        ("french_revolutionary", Box::new(FrenchRevolutionaryCalendar)),
        ("jewish", Box::new(JewishCalendar::new())),
    ]
    .into_iter()
    .collect();

    for calendar_system in &settings.enabled_calendars {
        if let Some(calendar) = calendars.get(calendar_system.as_str()) {
            let date = calendar.convert(&now, Some(&settings));
            dates.push(date);
        }
    }

    Ok(dates)
}

#[tauri::command]
pub fn get_available_calendar_plugins() -> Result<Vec<String>, String> {
    Ok(vec![
        "gregorian".to_string(),
        "julian".to_string(),
        "buddhist".to_string(),
        "french_revolutionary".to_string(),
        "jewish".to_string(),
    ])
}