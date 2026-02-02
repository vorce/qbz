//! Artist blacklist data models

use serde::{Deserialize, Serialize};

/// A blacklisted artist entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlacklistedArtist {
    pub artist_id: u64,
    pub artist_name: String,
    pub added_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Blacklist settings (enable/disable toggle)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlacklistSettings {
    pub enabled: bool,
}

impl Default for BlacklistSettings {
    fn default() -> Self {
        Self { enabled: true }
    }
}
