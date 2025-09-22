mod calendar;
mod calendar_commands;
mod commands;
mod models;
mod settings;
mod utils;

use tauri::Builder;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            #[cfg(desktop)]
            app.handle().plugin(tauri_plugin_global_shortcut::Builder::new().build())?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_system_username,
            commands::get_system_realname,
            commands::hide_window,
            commands::show_window,
            settings::load_settings,
            settings::save_settings,
            settings::verify_password,
            calendar_commands::get_current_dates,
            calendar_commands::get_available_calendar_plugins,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}