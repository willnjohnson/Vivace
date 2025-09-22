use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSettings {
    pub password: String,
    pub background_type: String,
    pub background_value: String,
    pub avatar_path: Option<String>,
    pub enabled_calendars: Vec<String>,
    pub timeout_minutes: Option<u32>,
    pub hotkey_combination: Option<String>,
    pub auto_lock_enabled: Option<bool>,
    pub auto_lock_minutes: Option<u32>,
    pub show_seconds: Option<bool>,
    pub date_format: Option<String>,
    pub theme: Option<String>,
    pub sound_enabled: Option<bool>,
    pub sound_file: Option<String>,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            password: "password".to_string(),
            background_type: "gradient".to_string(),
            background_value: "linear-gradient(135deg, #579945 0%, #764ba2 100%)".to_string(),
            avatar_path: Some("/fox_profile.png".to_string()),
            enabled_calendars: vec![
                "french_revolutionary".to_string(),
                "gregorian".to_string(),
                "julian".to_string(),
                "buddhist".to_string(),
                "jewish".to_string(),
            ],
            timeout_minutes: Some(1),
            hotkey_combination: Some("Alt+L".to_string()),
            auto_lock_enabled: None,
            auto_lock_minutes: None,
            show_seconds: Some(true),
            date_format: Some("military".to_string()),
            theme: None,
            sound_enabled: None,
            sound_file: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarDate {
    pub system: String,
    pub date: String,
    pub additional_info: Option<String>,
}