// commands.rs
// Maintains commands

use tauri::Window;

#[tauri::command]
pub fn get_system_username() -> String {
    whoami::username()
}

#[tauri::command]
pub fn get_system_realname() -> String {
    whoami::realname()
}

#[tauri::command]
pub async fn hide_window(window: Window) -> Result<(), String> {
    window.hide().map_err(|e| format!("Failed to hide window: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn show_window(window: Window) -> Result<(), String> {
    window.show().map_err(|e| format!("Failed to show window: {}", e))?;
    window.set_focus().map_err(|e| format!("Failed to focus window: {}", e))?;
    Ok(())
}