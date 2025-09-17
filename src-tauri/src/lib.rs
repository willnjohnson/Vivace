use chrono::{Datelike, DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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
            ],
            timeout_minutes: Some(1),
            hotkey_combination: Some("Alt+L".to_string()),
            auto_lock_enabled: None,
            auto_lock_minutes: None,
            show_seconds: Some(true), // or None
            date_format: Some("military".to_string()), // or "standard"
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

// Tauri commands
#[tauri::command]
fn get_system_username() -> String {
    whoami::username()
}

#[tauri::command]
fn get_system_realname() -> String {
    whoami::realname()
}

#[tauri::command]
fn load_settings() -> Result<UserSettings, String> {
    let settings_path = get_settings_path()?;
    println!("Attempting to load settings from: {:?}", settings_path); // Added debug print

    if settings_path.exists() {
        let content = fs::read_to_string(&settings_path).map_err(|e| {
            let error_msg =
                format!("Failed to read settings from {:?}: {}", settings_path, e);
            eprintln!("{}", error_msg); // Added error print
            error_msg
        })?;

        // Added debug print for content
        println!("Read settings content:\n{}", content);

        serde_json::from_str(&content).map_err(|e| {
            let error_msg =
                format!("Failed to parse settings from {:?}: {}", settings_path, e);
            eprintln!("{}", error_msg); // Added error print
            error_msg
        })
    } else {
        println!(
            "Settings file not found at {:?}. Creating default settings.",
            settings_path
        ); // Added debug print
        let default_settings = UserSettings::default();
        save_settings(default_settings.clone())?;
        Ok(default_settings)
    }
}

#[tauri::command]
fn save_settings(settings: UserSettings) -> Result<(), String> {
    let settings_path = get_settings_path()?;
    println!("Attempting to save settings to: {:?}", settings_path); // Added debug print

    if let Some(parent) = settings_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create settings directory: {}", e))?;
    }

    // Serialize the settings struct directly into a JSON string
    // serde_json::to_string_pretty adds formatting (indentation) for readability
    let json_content = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;

    println!("Saving settings content:\n{}", json_content); // Added debug print

    fs::write(&settings_path, json_content)
        .map_err(|e| format!("Failed to write settings: {}", e))?;

    Ok(())
}

#[tauri::command]
fn verify_password(password: String) -> Result<bool, String> {
    let settings = load_settings()?;
    Ok(settings.password == password)
}

#[tauri::command]
async fn hide_window(window: tauri::Window) -> Result<(), String> {
    window.hide().map_err(|e| format!("Failed to hide window: {}", e))?;
    Ok(())
}

#[tauri::command]
async fn show_window(window: tauri::Window) -> Result<(), String> {
    window.show().map_err(|e| format!("Failed to show window: {}", e))?;
    window.set_focus().map_err(|e| format!("Failed to focus window: {}", e))?;
    Ok(())
}

#[tauri::command]
fn get_current_dates() -> Result<Vec<CalendarDate>, String> {
    let settings = load_settings()?;
    let mut dates = Vec::new();
    let now = Local::now();

    for calendar_system in &settings.enabled_calendars {
        match calendar_system.as_str() {
            "gregorian" => {
                let format_str = resolve_date_format(
                    settings.date_format.as_deref(),
                    settings.show_seconds.unwrap_or(false),
                );
                // Debug print the resolved format string for Gregorian
                println!(
                    "Gregorian calendar: date_format={:?}, show_seconds={:?}, resolved_format_str={}",
                    settings.date_format,
                    settings.show_seconds,
                    format_str
                );

                dates.push(CalendarDate {
                    system: "Gregorian".to_string(),
                    date: now.format(&format_str).to_string(),
                    additional_info: None,
                });
            }
            "julian" => {
                let julian_date = convert_to_julian(&now);
                dates.push(CalendarDate {
                    system: "Julian".to_string(),
                    date: julian_date,
                    additional_info: None,
                });
            }
            "buddhist" => {
                let buddhist_date = convert_to_buddhist(&now);
                dates.push(CalendarDate {
                    system: "Buddhist".to_string(),
                    date: buddhist_date,
                    additional_info: None,
                });
            }
            "french_revolutionary" => {
                let (french_date, item) = convert_to_french_revolutionary(&now);
                dates.push(CalendarDate {
                    system: "French Revolutionary".to_string(),
                    date: french_date,
                    additional_info: Some(item),
                });
            }
            _ => {}
        }
    }

    Ok(dates)
}

#[tauri::command]
fn get_available_calendar_plugins() -> Result<Vec<String>, String> {
    // This would eventually load from a plugins directory
    Ok(vec![
        "gregorian".to_string(),
        "julian".to_string(),
        "buddhist".to_string(),
        "french_revolutionary".to_string(),
    ])
}

fn get_settings_path() -> Result<PathBuf, String> {
    let app_data_dir = dirs::config_dir().ok_or("Failed to get config directory")?;

    Ok(app_data_dir.join("Vivace").join("settings.json"))
}

fn resolve_date_format(user_value: Option<&str>, show_seconds: bool) -> String {
    let mut format = match user_value {
        Some("military") => "%A, %B %d, %Y %H:%M".to_string(),
        Some("standard") => "%A, %B %d, %Y %I:%M %p".to_string(),
        Some(custom) if custom.starts_with("custom:") => {
            custom.trim_start_matches("custom:").to_string()
        }
        _ => "%A, %B %d, %Y".to_string(), // fallback default (no time component)
    };

    if show_seconds && !format.contains("%S") {
        // Only append seconds if a time component (%H or %I) is already present
        // This prevents appending :%S to a date-only format like "%A, %B %d, %Y"
        if format.contains("%H") || format.contains("%I") {
            format.push_str(":%S");
        }
    }

    format
}

fn convert_to_julian(date: &DateTime<Local>) -> String {
    // Simplified Julian calendar conversion
    let gregorian_date = date.naive_local().date();
    let julian_offset = 13; // Approximate offset for 21st century
    let julian_date = gregorian_date - chrono::Duration::days(julian_offset);
    julian_date.format("%B %d, %Y").to_string()
}

fn convert_to_buddhist(date: &DateTime<Local>) -> String {
    // Buddhist calendar adds 543 years to Gregorian
    let buddhist_year = date.year() + 543;
    format!("{}, {} BE", date.format("%B %d"), buddhist_year)
}

fn convert_to_french_revolutionary(date: &DateTime<Local>) -> (String, String) {
    use chrono::{Datelike, NaiveDate};

    let naive = date.naive_local().date();
    let year = naive.year();

    // Approximation: Vendémiaire 1 = September 22
    let vend_start_this = NaiveDate::from_ymd_opt(year, 9, 22).unwrap();

    // If the date is before Sept 22, use previous year as start
    let (start_of_year, revolutionary_year) = if naive >= vend_start_this {
        (vend_start_this, year - 1792 + 1)
    } else {
        (NaiveDate::from_ymd_opt(year - 1, 9, 22).unwrap(), year - 1792)
    };

    let days_since_start = (naive - start_of_year).num_days();
    let month_index = (days_since_start / 30) as usize;
    let day_in_month = ((days_since_start % 30) + 1) as usize;

    let month_names = [
        "Vendémiaire",
        "Brumaire",
        "Frimaire",
        "Nivôse",
        "Pluviôse",
        "Ventôse",
        "Germinal",
        "Floréal",
        "Prairial",
        "Messidor",
        "Thermidor",
        "Fructidor",
    ];

    let month_name = if month_index < 12 {
        month_names[month_index].to_string()
    } else {
        "Sansculottides".to_string()
    };

    let date_str = format!("{} {}, An {}", month_name, day_in_month, revolutionary_year);

    let item = get_french_revolutionary_item(month_index, day_in_month);

    (date_str, item)
}

fn get_french_revolutionary_item(month: usize, day: usize) -> String {
    match month {
        0 => {
            // Vendémiaire
            let items = [
                "Raisin",
                "Safran",
                "Châtaigne",
                "Colchique",
                "Cheval",
                "Balsamine",
                "Carotte",
                "Amaranthe",
                "Panais",
                "Cuve",
                "Pomme de terre",
                "Immortelle",
                "Potiron",
                "Réséda",
                "Âne",
                "Belle de nuit",
                "Citrouille",
                "Sarrasin",
                "Tournesol",
                "Pressoir",
                "Chanvre",
                "Pêche",
                "Navet",
                "Amaryllis",
                "Bœuf",
                "Aubergine",
                "Piment",
                "Tomate",
                "Orge",
                "Tonneau",
            ];
            items.get(day - 1).unwrap_or(&"Item").to_string()
        }
        1 => {
            // Brumaire
            let items = [
                "Pomme",
                "Céleri",
                "Poire",
                "Betterave",
                "Oie",
                "Héliotrope",
                "Figue",
                "Scorsonère",
                "Alisier",
                "Charrue",
                "Salsifis",
                "Mâcre",
                "Topinambour",
                "Endive",
                "Dindon",
                "Chervis",
                "Cresson",
                "Dentelaire",
                "Grenade",
                "Herse",
                "Bacchante",
                "Azerole",
                "Garance",
                "Orange",
                "Faisan",
                "Pistache",
                "Macjonc",
                "Coing",
                "Cormier",
                "Rouleau",
            ];
            items.get(day - 1).unwrap_or(&"Item").to_string()
        }
        2 => {
            // Frimaire
            let items = [
                "Raiponce",
                "Turneps",
                "Chicorée",
                "Nèfle",
                "Cochon",
                "Mâche",
                "Chou-fleur",
                "Miel",
                "Genièvre",
                "Pioche",
                "Cire",
                "Raifort",
                "Cèdre",
                "Sapin",
                "Chevreuil",
                "Ajonc",
                "Cyprès",
                "Lierre",
                "Sabine",
                "Hoyau",
                "Érable à sucre",
                "Bruyère",
                "Roseau",
                "Oseille",
                "Grillon",
                "Pignon",
                "Liège",
                "Truffe",
                "Olive",
                "Pelle",
            ];
            items.get(day - 1).unwrap_or(&"Item").to_string()
        }
        3 => {
            // Nivôse
            let items = [
                "Tourbe",
                "Houille",
                "Bitume",
                "Soufre",
                "Chien",
                "Lave",
                "Terre végétale",
                "Fumier",
                "Salpêtre",
                "Fléau",
                "Granit",
                "Argile",
                "Ardoise",
                "Grès",
                "Lapin",
                "Silex",
                "Marne",
                "Pierre à chaux",
                "Marbre",
                "Van",
                "Pierre à plâtre",
                "Sel",
                "Fer",
                "Cuivre",
                "Chat",
                "Étain",
                "Plomb",
                "Zinc",
                "Mercure",
                "Crible",
            ];
            items.get(day - 1).unwrap_or(&"Item").to_string()
        }
        4 => {
            // Pluviôse
            let items = [
                "Lauréole",
                "Mousse",
                "Fragon",
                "Perce-neige",
                "Taureau",
                "Laurier-thym",
                "Amadouvier",
                "Mézéréon",
                "Peuplier",
                "Coignée",
                "Ellébore",
                "Brocoli",
                "Laurier",
                "Avelinier",
                "Vache",
                "Buis",
                "Lichen",
                "If",
                "Pulmonaire",
                "Serpette",
                "Thlaspi",
                "Thimelé",
                "Chiendent",
                "Trainasse",
                "Lièvre",
                "Guède",
                "Noisetier",
                "Cyclamen",
                "Chélidoine",
                "Traîneau",
            ];
            items.get(day - 1).unwrap_or(&"Item").to_string()
        }
        5 => {
            // Ventôse
            let items = [
                "Tussilage",
                "Cornouiller",
                "Violier",
                "Troène",
                "Bouc",
                "Asaret",
                "Alaterne",
                "Violette",
                "Marceau",
                "Bêche",
                "Narcisse",
                "Orme",
                "Fumeterre",
                "Vélar",
                "Chèvre",
                "Épinard",
                "Doronic",
                "Mouron",
                "Cerfeuil",
                "Cordeau",
                "Mandragore",
                "Persil",
                "Cochléaria",
                "Pâquerette",
                "Thon",
                "Pissenlit",
                "Sylvie",
                "Capillaire",
                "Frêne",
                "Plantoir",
            ];
            items.get(day - 1).unwrap_or(&"Item").to_string()
        }
        6 => {
            // Germinal
            let items = [
                "Primevère",
                "Platane",
                "Asperge",
                "Tulipe",
                "Poule",
                "Bette",
                "Bouleau",
                "Jonquille",
                "Aulne",
                "Couvoir",
                "Pervenche",
                "Charme",
                "Morille",
                "Hêtre",
                "Abeille",
                "Laitue",
                "Mélèze",
                "Ciguë",
                "Radis",
                "Ruche",
                "Gainier",
                "Romaine",
                "Marronnier",
                "Roquette",
                "Pigeon",
                "Lilas",
                "Anémone",
                "Pensée",
                "Myrtille",
                "Greffoir",
            ];
            items.get(day - 1).unwrap_or(&"Item").to_string()
        }
        7 => {
            // Floréal
            let items = [
                "Rose",
                "Chêne",
                "Fougère",
                "Aubépine",
                "Rossignol",
                "Ancolie",
                "Muguet",
                "Champignon",
                "Hyacinthe",
                "Râteau",
                "Rhubarbe",
                "Sainfoin",
                "Bâton d'or",
                "Chamerisier",
                "Ver à soie",
                "Consoude",
                "Pimprenelle",
                "Corbeille d'or",
                "Arroche",
                "Sarcloir",
                "Statice",
                "Fritillaire",
                "Bourrache",
                "Valériane",
                "Carpe",
                "Fusain",
                "Civette",
                "Buglosse",
                "Sénevé",
                "Houlette",
            ];
            items.get(day - 1).unwrap_or(&"Item").to_string()
        }
        8 => {
            // Prairial
            let items = [
                "Luzerne",
                "Hémérocalle",
                "Trèfle",
                "Angélique",
                "Canard",
                "Mélisse",
                "Fromental",
                "Martagon",
                "Serpolet",
                "Faux",
                "Fraise",
                "Bétoine",
                "Pois",
                "Acacia",
                "Caille",
                "Œillet",
                "Sureau",
                "Pavot",
                "Tilleul",
                "Fourche",
                "Barbeau",
                "Camomille",
                "Chèvrefeuille",
                "Caille-lait",
                "Tanche",
                "Jasmin",
                "Verveine",
                "Thym",
                "Pivoine",
                "Chariot",
            ];
            items.get(day - 1).unwrap_or(&"Item").to_string()
        }
        9 => {
            // Messidor
            let items = [
                "Seigle",
                "Avoine",
                "Oignon",
                "Véronique",
                "Mulet",
                "Romarin",
                "Concombre",
                "Échalote",
                "Absinthe",
                "Faucille",
                "Coriandre",
                "Artichaut",
                "Girofle",
                "Lavande",
                "Chamois",
                "Tabac",
                "Groseille",
                "Gesse",
                "Cerise",
                "Parc",
                "Menthe",
                "Cumin",
                "Haricot",
                "Orcanète",
                "Pintade",
                "Sauge",
                "Ail",
                "Vesce",
                "Blé",
                "Chalémie",
            ];
            items.get(day - 1).unwrap_or(&"Item").to_string()
        }
        10 => {
            // Thermidor
            let items = [
                "Épeautre",
                "Bouillon blanc",
                "Melon",
                "Ivraie",
                "Bélier",
                "Prêle",
                "Armoise",
                "Carthame",
                "Mûre",
                "Arrosoir",
                "Panic",
                "Salicorne",
                "Abricot",
                "Basilic",
                "Brebis",
                "Guimauve",
                "Lin",
                "Amande",
                "Gentiane",
                "Écluse",
                "Carline",
                "Câprier",
                "Lentille",
                "Aunée",
                "Loutre",
                "Myrte",
                "Colza",
                "Lupin",
                "Coton",
                "Moulin",
            ];
            items.get(day - 1).unwrap_or(&"Item").to_string()
        }
        11 => {
            // Fructidor
            let items = [
                "Prune",
                "Millet",
                "Lycoperdon",
                "Escourgeon",
                "Saumon",
                "Tubéreuse",
                "Sucrion",
                "Apocyn",
                "Réglisse",
                "Échelle",
                "Pastèque",
                "Fenouil",
                "Épine vinette",
                "Noix",
                "Truite",
                "Citron",
                "Cardère",
                "Nerprun",
                "Tagette",
                "Hotte",
                "Églantier",
                "Noisette",
                "Houblon",
                "Sorgho",
                "Écrevisse",
                "Bigarade",
                "Verge d'or",
                "Maïs",
                "Marron",
                "Panier",
            ];
            items.get(day - 1).unwrap_or(&"Item").to_string()
        }
        12 => {
            // Sans-culottides / complementary days
            match day {
                1 => "La Fête de la Vertu".to_string(),
                2 => "La Fête du Génie".to_string(),
                3 => "La Fête du Travail".to_string(),
                4 => "La Fête de l'Opinion".to_string(),
                5 => "La Fête des Récompenses".to_string(),
                6 => "La Fête de la Révolution".to_string(), // leap years only
                _ => "".to_string(),
            }
        }
        _ => format!("{}", day),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
            
            // Initialize global shortcut plugin on desktop platforms only
            #[cfg(desktop)]
            app.handle().plugin(tauri_plugin_global_shortcut::Builder::new().build())?;
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_system_username,
            get_system_realname,
            load_settings,
            save_settings,
            verify_password,
            get_current_dates,
            get_available_calendar_plugins,
            hide_window,
            show_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}