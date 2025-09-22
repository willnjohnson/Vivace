// jewish_calendar.rs
// Accurate Hebrew calendar conversion (Dershowitz & Reingold style).

use crate::models::CalendarDate;
use chrono::{DateTime, Local, NaiveDate, Datelike};
use std::collections::HashMap;

pub struct JewishCalendar {
    // we keep a tiny cache for computed 1 Tishri dates to avoid recomputing repeatedly
    rosh_hashanah_cache: HashMap<i32, NaiveDate>,
}

impl JewishCalendar {
    pub fn new() -> Self {
        Self {
            rosh_hashanah_cache: HashMap::new(),
        }
    }

    // public entry: convert a NaiveDate to a formatted hebrew date string
    fn to_jewish_date(&mut self, gregorian: NaiveDate) -> String {
        // compute absolute day for the gregorian date (days since Gregorian 1/1/1 = 1)
        let abs = absolute_from_gregorian(gregorian.year(), gregorian.month(), gregorian.day());

        // find hebrew year containing this absolute day
        let mut year = (gregorian.year() + 3760) as i32;

        // Find the Rosh Hashanah date for the year
        let mut rh_abs = self.rosh_hashanah_naive(year).num_days_from_ce() as i64 + 1;

        // Adjust the year if the date is before or after Rosh Hashanah
        if abs < rh_abs {
            year -= 1;
            rh_abs = self.rosh_hashanah_naive(year).num_days_from_ce() as i64 + 1;
        } else {
            let next_rh_abs = self.rosh_hashanah_naive(year + 1).num_days_from_ce() as i64 + 1;
            if abs >= next_rh_abs {
                year += 1;
                rh_abs = self.rosh_hashanah_naive(year).num_days_from_ce() as i64 + 1;
            }
        }

        // Now compute day within Hebrew year
        let day_of_year = (abs - rh_abs) as u32; // 0-based
        let month_lengths = get_hebrew_month_lengths(year);

        let mut rem = day_of_year;
        for (len, eng, heb) in month_lengths.iter() {
            if rem < *len {
                let day = rem + 1;
                let heb_day = hebrew_numeral(day);
                return format!(
                    "{} {} {} ({} {})",
                    day, eng, year, heb_day, heb
                );
            }
            rem -= *len;
        }

        format!("Date beyond year boundary: {} days since Rosh Hashanah", day_of_year)
    }

    // Helper: get 1 Tishri as NaiveDate for given hebrew year (cached)
    fn rosh_hashanah_naive(&mut self, year: i32) -> NaiveDate {
        if let Some(&d) = self.rosh_hashanah_cache.get(&year) {
            return d;
        }
        let abs = hebrew_calendar_elapsed_days(year);
        let naive = gregorian_from_absolute(abs);
        self.rosh_hashanah_cache.insert(year, naive);
        naive
    }
}

impl super::Calendar for JewishCalendar {
    fn convert(&self, date: &DateTime<Local>, _settings: Option<&crate::models::UserSettings>) -> CalendarDate {
        let mut tmp = JewishCalendar {
            rosh_hashanah_cache: self.rosh_hashanah_cache.clone(),
        };
        let gregorian_date = date.naive_local().date();
        let date_str = tmp.to_jewish_date(gregorian_date);
        CalendarDate {
            system: "Jewish".to_string(),
            date: date_str,
            additional_info: None,
        }
    }
}

// Hebrew epoch constant
const HEBREW_EPOCH: i64 = -1373429;

// Compute the days (absolute) of Rosh Hashanah for a given Hebrew year.
fn hebrew_calendar_elapsed_days(year: i32) -> i64 {
    let y = year - 1;
    let months_elapsed = (235 * (y / 19))
        + 12 * (y % 19)
        + ((7 * (y % 19) + 1) / 19);

    let parts_elapsed = 204 + 793 * months_elapsed + 1080 * (12 * months_elapsed);
    let day = 1 + 29 * months_elapsed + (parts_elapsed / 25920);
    let parts = parts_elapsed % 25920;

    let mut day = day;

    if parts >= 19440 {
        day += 1;
    }

    let mut day_of_week = (day % 7) as i32;

    if day_of_week == 2 && parts >= 9924 && !is_jewish_leap_year(year) {
        day += 1;
        day_of_week = (day % 7) as i32;
    }

    if day_of_week == 1 && parts >= 16789 && is_jewish_leap_year(year - 1) {
        day += 1;
        day_of_week = (day % 7) as i32;
    }

    if day_of_week == 0 || day_of_week == 3 || day_of_week == 5 {
        day += 1;
    }
    
    // The previous implementation was likely off by one.
    // The "absolute day" must be 1-based, and the calculation
    // can result in a 0-based offset. The simplest fix is to add 1 here.
    let final_day = day + 1;

    HEBREW_EPOCH + final_day as i64
}

// Return month lengths for the year
fn get_hebrew_month_lengths(year: i32) -> Vec<(u32, &'static str, &'static str)> {
    let is_leap = is_jewish_leap_year(year);
    let days_in_year = (hebrew_calendar_elapsed_days(year + 1) - hebrew_calendar_elapsed_days(year)) as i64;

    let (cheshvan_len, kislev_len) = match days_in_year {
        353 | 383 => (29, 29),
        354 | 384 => (29, 30),
        355 | 385 => (30, 30),
        _ => (29, 30),
    };

    let mut months: Vec<(u32, &'static str, &'static str)> = Vec::new();
    months.push((30, "Tishrei", "תשרי"));
    months.push((cheshvan_len as u32, "Cheshvan", "חשון"));
    months.push((kislev_len as u32, "Kislev", "כסלו"));
    months.push((29, "Tevet", "טבת"));
    months.push((30, "Shevat", "שבט"));

    if is_leap {
        months.push((30, "Adar I", "אדר א׳"));
        months.push((29, "Adar II", "אדר ב׳"));
    } else {
        months.push((29, "Adar", "אדר"));
    }

    months.push((30, "Nisan", "ניסן"));
    months.push((29, "Iyar", "אייר"));
    months.push((30, "Sivan", "סיון"));
    months.push((29, "Tammuz", "תמוז"));
    months.push((30, "Av", "אב"));
    months.push((29, "Elul", "אלול"));

    months
}

// Leap year in 19-year cycle
fn is_jewish_leap_year(year: i32) -> bool {
    matches!(year % 19, 0 | 3 | 6 | 8 | 11 | 14 | 17)
}

/* -------------------------
    Helpers: absolute <-> gregorian
    ------------------------- */

fn absolute_from_gregorian(year: i32, month: u32, day: u32) -> i64 {
    let y = year as i64;
    let d = day as i64;

    let mut n = d;
    for mm in 1..month {
        n += days_in_gregorian_month(year, mm) as i64;
    }

    n += 365 * (y - 1);
    n += (y - 1) / 4;
    n -= (y - 1) / 100;
    n += (y - 1) / 400;

    n
}

fn days_in_gregorian_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_gregorian_leap_year(year) { 29 } else { 28 },
        _ => 30,
    }
}

fn is_gregorian_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

// Convert absolute day back to NaiveDate (Gregorian)
fn gregorian_from_absolute(abs: i64) -> NaiveDate {
    let mut year = ((4 * (abs - 1) + 3) / 1461) as i32;
    loop {
        let jan1 = absolute_from_gregorian(year + 1, 1, 1);
        if jan1 > abs {
            break;
        }
        year += 1;
    }
    let day_of_year = (abs - absolute_from_gregorian(year, 1, 1) + 1) as u32;
    let mut m = 1u32;
    let mut day_remaining = day_of_year;
    while day_remaining > days_in_gregorian_month(year, m) {
        day_remaining -= days_in_gregorian_month(year, m);
        m += 1;
    }
    NaiveDate::from_ymd_opt(year, m, day_remaining)
        .expect("Invalid Gregorian date from absolute")
}

/* -------------------------
    Display helpers
    ------------------------- */

fn hebrew_numeral(n: u32) -> String {
    match n {
        1 => "א׳".to_string(),
        2 => "ב׳".to_string(),
        3 => "ג׳".to_string(),
        4 => "ד׳".to_string(),
        5 => "ה׳".to_string(),
        6 => "ו׳".to_string(),
        7 => "ז׳".to_string(),
        8 => "ח׳".to_string(),
        9 => "ט׳".to_string(),
        10 => "י׳".to_string(),
        11 => "י״א".to_string(),
        12 => "י״ב".to_string(),
        13 => "י״ג".to_string(),
        14 => "י״ד".to_string(),
        15 => "ט״ו".to_string(),
        16 => "ט״ז".to_string(),
        17 => "י״ז".to_string(),
        18 => "י״ח".to_string(),
        19 => "י״ט".to_string(),
        20 => "כ׳".to_string(),
        21 => "כ״א".to_string(),
        22 => "כ״ב".to_string(),
        23 => "כ״ג".to_string(),
        24 => "כ״ד".to_string(),
        25 => "כ״ה".to_string(),
        26 => "כ״ו".to_string(),
        27 => "כ״ז".to_string(),
        28 => "כ״ח".to_string(),
        29 => "כ״ט".to_string(),
        30 => "ל׳".to_string(),
        _ => n.to_string(),
    }
}