use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PostMetadata {
    pub title: String,
    pub subtitle: Option<String>,
    pub slug: Option<String>,
    pub date: Option<DateTime<Utc>>,
}

impl PostMetadata {
    pub fn get_friendly_date(&self) -> Option<String> {
        self.date.as_ref().map(|date| date.format("%v").to_string())
    }

    pub fn get_html_time_datetime(&self) -> Option<String> {
        self.date
            .as_ref()
            .map(|date| date.format("%Y-%m-%dT%H:%MZ").to_string())
    }

    pub fn get_rfc2822_datetime(&self) -> Option<String> {
        self.date.as_ref().map(|date| date.to_rfc2822())
    }
}
