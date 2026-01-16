//! Offline mode detection and settings
//!
//! Handles:
//! - Network connectivity detection
//! - Login state checking
//! - Manual offline mode toggle
//! - Offline settings persistence
//! - Pending playlist sync queue (playlists created offline)

use rusqlite::{Connection, params, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, Emitter};

/// Reason why the app is in offline mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OfflineReason {
    NoNetwork,
    NotLoggedIn,
    ManualOverride,
}

/// Current offline status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfflineStatus {
    pub is_offline: bool,
    pub reason: Option<OfflineReason>,
    pub manual_mode_enabled: bool,
}

/// Persistent offline settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OfflineSettings {
    pub manual_offline_mode: bool,
    pub show_partial_playlists: bool,
}

/// A playlist created offline, pending sync to Qobuz
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingPlaylist {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub track_ids: Vec<u64>,
    pub created_at: i64,
    pub synced: bool,
    pub qobuz_playlist_id: Option<u64>,
}

/// SQLite-backed storage for offline settings
pub struct OfflineStore {
    conn: Connection,
}

impl OfflineStore {
    pub fn new() -> Result<Self, String> {
        let data_dir = dirs::data_dir()
            .ok_or("Could not determine data directory")?
            .join("qbz");

        std::fs::create_dir_all(&data_dir)
            .map_err(|e| format!("Failed to create data directory: {}", e))?;

        let db_path = data_dir.join("offline_settings.db");
        let conn = Connection::open(&db_path)
            .map_err(|e| format!("Failed to open offline settings database: {}", e))?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS offline_settings (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                manual_offline_mode INTEGER NOT NULL DEFAULT 0,
                show_partial_playlists INTEGER NOT NULL DEFAULT 1
            );
            INSERT OR IGNORE INTO offline_settings (id, manual_offline_mode, show_partial_playlists)
            VALUES (1, 0, 1);

            CREATE TABLE IF NOT EXISTS pending_playlist_sync (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT,
                is_public INTEGER NOT NULL DEFAULT 0,
                track_ids TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                synced INTEGER NOT NULL DEFAULT 0,
                qobuz_playlist_id INTEGER
            );
            CREATE INDEX IF NOT EXISTS idx_pending_playlist_synced ON pending_playlist_sync(synced);"
        ).map_err(|e| format!("Failed to create offline settings table: {}", e))?;

        Ok(Self { conn })
    }

    pub fn get_settings(&self) -> Result<OfflineSettings, String> {
        self.conn
            .query_row(
                "SELECT manual_offline_mode, show_partial_playlists FROM offline_settings WHERE id = 1",
                [],
                |row| {
                    Ok(OfflineSettings {
                        manual_offline_mode: row.get::<_, i64>(0)? != 0,
                        show_partial_playlists: row.get::<_, i64>(1)? != 0,
                    })
                },
            )
            .map_err(|e| format!("Failed to get offline settings: {}", e))
    }

    pub fn set_manual_offline_mode(&self, enabled: bool) -> Result<(), String> {
        self.conn
            .execute(
                "UPDATE offline_settings SET manual_offline_mode = ?1 WHERE id = 1",
                params![enabled as i64],
            )
            .map_err(|e| format!("Failed to set manual offline mode: {}", e))?;
        Ok(())
    }

    pub fn set_show_partial_playlists(&self, enabled: bool) -> Result<(), String> {
        self.conn
            .execute(
                "UPDATE offline_settings SET show_partial_playlists = ?1 WHERE id = 1",
                params![enabled as i64],
            )
            .map_err(|e| format!("Failed to set show partial playlists: {}", e))?;
        Ok(())
    }

    // === Pending Playlist Sync Methods ===

    /// Create a new pending playlist (created while offline)
    pub fn create_pending_playlist(
        &self,
        name: &str,
        description: Option<&str>,
        is_public: bool,
        track_ids: &[u64],
    ) -> Result<i64, String> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        let track_ids_json = serde_json::to_string(track_ids)
            .map_err(|e| format!("Failed to serialize track IDs: {}", e))?;

        self.conn
            .execute(
                "INSERT INTO pending_playlist_sync (name, description, is_public, track_ids, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![name, description, is_public as i64, track_ids_json, now],
            )
            .map_err(|e| format!("Failed to create pending playlist: {}", e))?;

        Ok(self.conn.last_insert_rowid())
    }

    /// Get all pending (unsynced) playlists
    pub fn get_pending_playlists(&self) -> Result<Vec<PendingPlaylist>, String> {
        let mut stmt = self.conn
            .prepare(
                "SELECT id, name, description, is_public, track_ids, created_at, synced, qobuz_playlist_id
                 FROM pending_playlist_sync WHERE synced = 0 ORDER BY created_at ASC"
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let playlists = stmt
            .query_map([], |row| {
                let track_ids_json: String = row.get(4)?;
                let track_ids: Vec<u64> = serde_json::from_str(&track_ids_json).unwrap_or_default();

                Ok(PendingPlaylist {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    is_public: row.get::<_, i64>(3)? != 0,
                    track_ids,
                    created_at: row.get(5)?,
                    synced: row.get::<_, i64>(6)? != 0,
                    qobuz_playlist_id: row.get::<_, Option<i64>>(7)?.map(|id| id as u64),
                })
            })
            .map_err(|e| format!("Failed to query pending playlists: {}", e))?;

        playlists
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect pending playlists: {}", e))
    }

    /// Mark a pending playlist as synced with its Qobuz ID
    pub fn mark_playlist_synced(&self, pending_id: i64, qobuz_playlist_id: u64) -> Result<(), String> {
        self.conn
            .execute(
                "UPDATE pending_playlist_sync SET synced = 1, qobuz_playlist_id = ?1 WHERE id = ?2",
                params![qobuz_playlist_id as i64, pending_id],
            )
            .map_err(|e| format!("Failed to mark playlist as synced: {}", e))?;
        Ok(())
    }

    /// Delete a pending playlist (e.g., if user cancels before sync)
    pub fn delete_pending_playlist(&self, pending_id: i64) -> Result<(), String> {
        self.conn
            .execute(
                "DELETE FROM pending_playlist_sync WHERE id = ?1",
                params![pending_id],
            )
            .map_err(|e| format!("Failed to delete pending playlist: {}", e))?;
        Ok(())
    }

    /// Get count of pending playlists
    pub fn get_pending_playlist_count(&self) -> Result<u32, String> {
        self.conn
            .query_row(
                "SELECT COUNT(*) FROM pending_playlist_sync WHERE synced = 0",
                [],
                |row| row.get::<_, i64>(0),
            )
            .map(|count| count as u32)
            .map_err(|e| format!("Failed to count pending playlists: {}", e))
    }
}

/// Thread-safe wrapper for OfflineStore
pub struct OfflineState {
    pub store: Arc<Mutex<OfflineStore>>,
}

impl OfflineState {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            store: Arc::new(Mutex::new(OfflineStore::new()?)),
        })
    }
}

/// Check network connectivity by attempting to reach Qobuz API
pub async fn check_network_connectivity() -> bool {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build();

    let client = match client {
        Ok(c) => c,
        Err(_) => return false,
    };

    // Use HEAD request for minimal data transfer
    match client.head("https://www.qobuz.com").send().await {
        Ok(response) => response.status().is_success() || response.status().is_redirection(),
        Err(_) => false,
    }
}

// Tauri commands
pub mod commands {
    use super::*;
    use crate::AppState;
    use tauri::State;

    /// Get current offline status
    #[tauri::command]
    pub async fn get_offline_status(
        offline_state: State<'_, OfflineState>,
        app_state: State<'_, AppState>,
    ) -> Result<OfflineStatus, String> {
        let settings = {
            let store = offline_state.store.lock().map_err(|e| format!("Lock error: {}", e))?;
            store.get_settings()?
        };

        // If manual offline mode is enabled, return immediately
        if settings.manual_offline_mode {
            return Ok(OfflineStatus {
                is_offline: true,
                reason: Some(OfflineReason::ManualOverride),
                manual_mode_enabled: true,
            });
        }

        // Check if user is logged in
        let is_logged_in = {
            let client = app_state.client.lock().await;
            client.is_logged_in().await
        };

        if !is_logged_in {
            return Ok(OfflineStatus {
                is_offline: true,
                reason: Some(OfflineReason::NotLoggedIn),
                manual_mode_enabled: false,
            });
        }

        // Check network connectivity
        let has_network = check_network_connectivity().await;

        if !has_network {
            return Ok(OfflineStatus {
                is_offline: true,
                reason: Some(OfflineReason::NoNetwork),
                manual_mode_enabled: false,
            });
        }

        Ok(OfflineStatus {
            is_offline: false,
            reason: None,
            manual_mode_enabled: false,
        })
    }

    /// Get offline settings
    #[tauri::command]
    pub fn get_offline_settings(
        state: State<'_, OfflineState>,
    ) -> Result<OfflineSettings, String> {
        let store = state.store.lock().map_err(|e| format!("Lock error: {}", e))?;
        store.get_settings()
    }

    /// Enable or disable manual offline mode
    #[tauri::command]
    pub async fn set_manual_offline(
        enabled: bool,
        state: State<'_, OfflineState>,
        app_state: State<'_, AppState>,
        app_handle: AppHandle,
    ) -> Result<OfflineStatus, String> {
        {
            let store = state.store.lock().map_err(|e| format!("Lock error: {}", e))?;
            store.set_manual_offline_mode(enabled)?;
        }

        // Get updated status
        let status = get_offline_status(state, app_state).await?;

        // Emit event to frontend
        let _ = app_handle.emit("offline-status-changed", &status);

        Ok(status)
    }

    /// Set whether to show playlists with partial local content in offline mode
    #[tauri::command]
    pub fn set_show_partial_playlists(
        enabled: bool,
        state: State<'_, OfflineState>,
    ) -> Result<(), String> {
        let store = state.store.lock().map_err(|e| format!("Lock error: {}", e))?;
        store.set_show_partial_playlists(enabled)
    }

    // === Pending Playlist Sync Commands ===

    /// Create a playlist while offline (queued for sync)
    #[tauri::command]
    pub fn create_pending_playlist(
        name: String,
        description: Option<String>,
        is_public: bool,
        track_ids: Vec<u64>,
        state: State<'_, OfflineState>,
    ) -> Result<i64, String> {
        let store = state.store.lock().map_err(|e| format!("Lock error: {}", e))?;
        store.create_pending_playlist(&name, description.as_deref(), is_public, &track_ids)
    }

    /// Get all playlists pending sync
    #[tauri::command]
    pub fn get_pending_playlists(
        state: State<'_, OfflineState>,
    ) -> Result<Vec<PendingPlaylist>, String> {
        let store = state.store.lock().map_err(|e| format!("Lock error: {}", e))?;
        store.get_pending_playlists()
    }

    /// Get count of pending playlists
    #[tauri::command]
    pub fn get_pending_playlist_count(
        state: State<'_, OfflineState>,
    ) -> Result<u32, String> {
        let store = state.store.lock().map_err(|e| format!("Lock error: {}", e))?;
        store.get_pending_playlist_count()
    }

    /// Mark a pending playlist as synced after successful Qobuz creation
    #[tauri::command]
    pub fn mark_pending_playlist_synced(
        pending_id: i64,
        qobuz_playlist_id: u64,
        state: State<'_, OfflineState>,
    ) -> Result<(), String> {
        let store = state.store.lock().map_err(|e| format!("Lock error: {}", e))?;
        store.mark_playlist_synced(pending_id, qobuz_playlist_id)
    }

    /// Delete a pending playlist
    #[tauri::command]
    pub fn delete_pending_playlist(
        pending_id: i64,
        state: State<'_, OfflineState>,
    ) -> Result<(), String> {
        let store = state.store.lock().map_err(|e| format!("Lock error: {}", e))?;
        store.delete_pending_playlist(pending_id)
    }
}
