use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PostMetadata {
    pub title: String,
    pub subtitle: Option<String>,
    pub slug: Option<String>,
    pub date: Option<DateTime<Utc>>,
}
