use crate::models::UserSettings;
use std::fs;
use std::path::PathBuf;

pub fn get_settings_path() -> Result<PathBuf, String> {
    let app_data_dir = dirs::config_dir().ok_or("Failed to get config directory")?;
    Ok(app_data_dir.join("Vivace").join("settings.json"))
}

#[tauri::command]
pub fn load_settings() -> Result<UserSettings, String> {
    let settings_path = get_settings_path()?;
    println!("Attempting to load settings from: {:?}", settings_path);

    if settings_path.exists() {
        let content = fs::read_to_string(&settings_path).map_err(|e| {
            let error_msg = format!("Failed to read settings from {:?}: {}", settings_path, e);
            eprintln!("{}", error_msg);
            error_msg
        })?;

        println!("Read settings content:\n{}", content);

        serde_json::from_str(&content).map_err(|e| {
            let error_msg = format!("Failed to parse settings from {:?}: {}", settings_path, e);
            eprintln!("{}", error_msg);
            error_msg
        })
    } else {
        println!("Settings file not found at {:?}. Creating default settings.", settings_path);
        let default_settings = UserSettings::default();
        save_settings(default_settings.clone())?;
        Ok(default_settings)
    }
}

#[tauri::command]
pub fn save_settings(settings: UserSettings) -> Result<(), String> {
    let settings_path = get_settings_path()?;
    println!("Attempting to save settings to: {:?}", settings_path);

    if let Some(parent) = settings_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create settings directory: {}", e))?;
    }

    let json_content = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;

    println!("Saving settings content:\n{}", json_content);

    fs::write(&settings_path, json_content)
        .map_err(|e| format!("Failed to write settings: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn verify_password(password: String) -> Result<bool, String> {
    let settings = load_settings()?;
    Ok(settings.password == password)
}