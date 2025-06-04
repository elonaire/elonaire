use chrono::{DateTime, Utc};

/// The `start` argument to be a valid RFC3339 formatted string.
pub fn get_elapsed_time(start: &String, end: &DateTime<Utc>) -> String {
    match DateTime::parse_from_rfc3339(start) {
        Ok(start) => match end.signed_duration_since(&start).num_weeks() {
            0 => match end.signed_duration_since(&start).num_days() {
                0 => match end.signed_duration_since(&start).num_hours() {
                    0 => match end.signed_duration_since(&start).num_minutes() {
                        0 => match end.signed_duration_since(&start).num_seconds() {
                            0 => "Just now".to_string(),
                            val => format!("{} seconds", val.to_string()),
                        },
                        val => format!("{} minutes", val.to_string()),
                    },
                    val => format!("{} hours", val.to_string()),
                },
                val => format!("{} days", val.to_string()),
            },
            val => format!("{} weeks", val.to_string()),
        },
        Err(_) => "Invalid Date".to_string(),
    }
}
