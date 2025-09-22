pub fn resolve_date_format(user_value: Option<&str>, show_seconds: bool) -> String {
    let mut format = match user_value {
        Some("military") => "%A, %B %d, %Y %H:%M".to_string(),
        Some("standard") => "%A, %B %d, %Y %I:%M %p".to_string(),
        Some(custom) if custom.starts_with("custom:") => custom.trim_start_matches("custom:").to_string(),
        _ => "%A, %B %d, %Y".to_string(),
    };

    if show_seconds && !format.contains("%S") {
        if format.contains("%H") || format.contains("%I") {
            format.push_str(":%S");
        }
    }

    format
}