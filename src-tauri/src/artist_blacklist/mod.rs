//! Artist blacklist module
//!
//! Provides local artist blacklist functionality with:
//! - O(1) lookup performance via in-memory HashSet
//! - SQLite persistence
//! - Global enable/disable toggle (feature flag)
//!
//! ## Usage
//!
//! The blacklist service is initialized at app startup and registered
//! as a Tauri managed state. Use the `is_blacklisted()` method to check
//! if an artist should be filtered.
//!
//! ```rust,ignore
//! if state.blacklist.is_blacklisted(artist_id) {
//!     // Skip this artist
//!     continue;
//! }
//! ```

pub mod models;
pub mod service;

pub use models::{BlacklistSettings, BlacklistedArtist};
pub use service::BlacklistService;

use std::sync::Mutex;

/// Thread-safe state wrapper for the blacklist service
pub struct BlacklistState {
    pub service: Mutex<BlacklistService>,
}

impl BlacklistState {
    /// Create a new blacklist state
    pub fn new() -> Result<Self, String> {
        let data_dir = dirs::data_dir()
            .ok_or("Could not determine data directory")?
            .join("qbz");

        std::fs::create_dir_all(&data_dir)
            .map_err(|e| format!("Failed to create data directory: {}", e))?;

        let db_path = data_dir.join("artist_blacklist.db");
        let service = BlacklistService::new(&db_path)?;

        Ok(Self {
            service: Mutex::new(service),
        })
    }

    /// Check if an artist is blacklisted - thread-safe O(1) operation
    #[inline]
    pub fn is_blacklisted(&self, artist_id: u64) -> bool {
        self.service
            .lock()
            .map(|s| s.is_blacklisted(artist_id))
            .unwrap_or(false)
    }

    /// Check if the feature is enabled
    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.service
            .lock()
            .map(|s| s.is_enabled())
            .unwrap_or(true)
    }

    /// Add an artist to the blacklist
    pub fn add(&self, artist_id: u64, artist_name: &str, notes: Option<&str>) -> Result<(), String> {
        self.service
            .lock()
            .map_err(|_| "Failed to acquire lock")?
            .add(artist_id, artist_name, notes)
    }

    /// Remove an artist from the blacklist
    pub fn remove(&self, artist_id: u64) -> Result<(), String> {
        self.service
            .lock()
            .map_err(|_| "Failed to acquire lock")?
            .remove(artist_id)
    }

    /// Get all blacklisted artists
    pub fn get_all(&self) -> Result<Vec<BlacklistedArtist>, String> {
        self.service
            .lock()
            .map_err(|_| "Failed to acquire lock")?
            .get_all()
    }

    /// Set enabled state
    pub fn set_enabled(&self, enabled: bool) -> Result<(), String> {
        self.service
            .lock()
            .map_err(|_| "Failed to acquire lock")?
            .set_enabled(enabled)
    }

    /// Get blacklist settings
    pub fn get_settings(&self) -> Result<BlacklistSettings, String> {
        self.service
            .lock()
            .map(|s| s.get_settings())
            .map_err(|_| "Failed to acquire lock".to_string())
    }

    /// Get count of blacklisted artists
    pub fn count(&self) -> usize {
        self.service.lock().map(|s| s.count()).unwrap_or(0)
    }

    /// Clear all blacklisted artists
    pub fn clear_all(&self) -> Result<(), String> {
        self.service
            .lock()
            .map_err(|_| "Failed to acquire lock")?
            .clear_all()
    }
}
