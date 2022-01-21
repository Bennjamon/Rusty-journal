use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    text: String,
    #[serde(with = "ts_seconds")]
    created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Self {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<20} [{}]", self.text, created_at)
    }
}
