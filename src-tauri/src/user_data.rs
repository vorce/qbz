//! Per-user data path management
//!
//! Each Qobuz user gets their own subdirectory under the app's data/cache paths.
//! This module provides the central path provider that all per-user state modules
//! use to determine where to store their databases.

use std::path::PathBuf;
use std::sync::RwLock;

/// Central path provider for per-user data isolation.
///
/// Holds the current user_id and provides methods to get
/// user-scoped data and cache directories.
pub struct UserDataPaths {
    user_id: RwLock<Option<u64>>,
}

impl UserDataPaths {
    pub fn new() -> Self {
        Self {
            user_id: RwLock::new(None),
        }
    }

    /// Set the current user after login
    pub fn set_user(&self, user_id: u64) {
        *self.user_id.write().expect("UserDataPaths write lock poisoned") = Some(user_id);
        log::info!("UserDataPaths: active user set to {}", user_id);
    }

    /// Clear the current user on logout
    pub fn clear_user(&self) {
        *self.user_id.write().expect("UserDataPaths write lock poisoned") = None;
        log::info!("UserDataPaths: active user cleared");
    }

    /// Get the current user ID, if set
    pub fn current_user_id(&self) -> Option<u64> {
        *self.user_id.read().expect("UserDataPaths read lock poisoned")
    }

    /// Get the user-scoped data directory: ~/.local/share/qbz/users/{uid}/
    pub fn user_data_dir(&self) -> Result<PathBuf, String> {
        let uid = self.user_id.read()
            .map_err(|e| format!("UserDataPaths read lock error: {}", e))?
            .ok_or("No active user - please log in")?;

        let base = dirs::data_dir()
            .ok_or("Could not determine data directory")?
            .join("qbz")
            .join("users")
            .join(uid.to_string());

        Ok(base)
    }

    /// Get the user-scoped cache directory: ~/.cache/qbz/users/{uid}/
    pub fn user_cache_dir(&self) -> Result<PathBuf, String> {
        let uid = self.user_id.read()
            .map_err(|e| format!("UserDataPaths read lock error: {}", e))?
            .ok_or("No active user - please log in")?;

        let base = dirs::cache_dir()
            .ok_or("Could not determine cache directory")?
            .join("qbz")
            .join("users")
            .join(uid.to_string());

        Ok(base)
    }

    /// Get the global (non-user-scoped) data directory: ~/.local/share/qbz/
    pub fn global_data_dir() -> Result<PathBuf, String> {
        dirs::data_dir()
            .ok_or_else(|| "Could not determine data directory".to_string())
            .map(|d| d.join("qbz"))
    }

    /// Get the global (non-user-scoped) cache directory: ~/.cache/qbz/
    pub fn global_cache_dir() -> Result<PathBuf, String> {
        dirs::cache_dir()
            .ok_or_else(|| "Could not determine cache directory".to_string())
            .map(|d| d.join("qbz"))
    }
}
