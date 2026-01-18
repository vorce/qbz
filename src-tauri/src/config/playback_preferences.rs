//! Playback preferences
//!
//! Stores user preferences for playback behavior (autoplay mode, etc.)

use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutoplayMode {
    /// Continue playing within the source (album, playlist, etc.)
    ContinueWithinSource,
    /// Play only the selected track, then stop
    PlayTrackOnly,
}

impl Default for AutoplayMode {
    fn default() -> Self {
        Self::ContinueWithinSource
    }
}

impl AutoplayMode {
    fn to_db_value(&self) -> &'static str {
        match self {
            AutoplayMode::ContinueWithinSource => "continue",
            AutoplayMode::PlayTrackOnly => "track_only",
        }
    }

    fn from_db_value(s: &str) -> Self {
        match s {
            "track_only" => AutoplayMode::PlayTrackOnly,
            _ => AutoplayMode::ContinueWithinSource,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackPreferences {
    pub autoplay_mode: AutoplayMode,
}

impl Default for PlaybackPreferences {
    fn default() -> Self {
        Self {
            autoplay_mode: AutoplayMode::ContinueWithinSource,
        }
    }
}

pub struct PlaybackPreferencesStore {
    conn: Connection,
}

impl PlaybackPreferencesStore {
    pub fn new() -> Result<Self, String> {
        let data_dir = dirs::data_dir()
            .ok_or("Could not determine data directory")?
            .join("qbz");

        std::fs::create_dir_all(&data_dir)
            .map_err(|e| format!("Failed to create data directory: {}", e))?;

        let db_path = data_dir.join("playback_preferences.db");
        let conn = Connection::open(&db_path)
            .map_err(|e| format!("Failed to open playback preferences database: {}", e))?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS playback_preferences (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                autoplay_mode TEXT NOT NULL DEFAULT 'continue'
            );
            INSERT OR IGNORE INTO playback_preferences (id, autoplay_mode)
            VALUES (1, 'continue');"
        ).map_err(|e| format!("Failed to create playback preferences table: {}", e))?;

        Ok(Self { conn })
    }

    pub fn get_preferences(&self) -> Result<PlaybackPreferences, String> {
        self.conn
            .query_row(
                "SELECT autoplay_mode FROM playback_preferences WHERE id = 1",
                [],
                |row| {
                    let autoplay_str: String = row.get(0)?;
                    Ok(PlaybackPreferences {
                        autoplay_mode: AutoplayMode::from_db_value(&autoplay_str),
                    })
                },
            )
            .map_err(|e| format!("Failed to get playback preferences: {}", e))
    }

    pub fn set_autoplay_mode(&self, mode: AutoplayMode) -> Result<(), String> {
        self.conn
            .execute(
                "UPDATE playback_preferences SET autoplay_mode = ?1 WHERE id = 1",
                params![mode.to_db_value()],
            )
            .map_err(|e| format!("Failed to set autoplay mode: {}", e))?;
        Ok(())
    }
}

/// Global state wrapper for thread-safe access
pub struct PlaybackPreferencesState {
    store: Arc<Mutex<PlaybackPreferencesStore>>,
}

impl PlaybackPreferencesState {
    pub fn new() -> Result<Self, String> {
        let store = PlaybackPreferencesStore::new()?;
        Ok(Self {
            store: Arc::new(Mutex::new(store)),
        })
    }

    pub fn get_preferences(&self) -> Result<PlaybackPreferences, String> {
        self.store.lock().unwrap().get_preferences()
    }

    pub fn set_autoplay_mode(&self, mode: AutoplayMode) -> Result<(), String> {
        self.store.lock().unwrap().set_autoplay_mode(mode)
    }
}

// Tauri commands

#[tauri::command]
pub fn get_playback_preferences(
    state: tauri::State<PlaybackPreferencesState>,
) -> Result<PlaybackPreferences, String> {
    state.get_preferences()
}

#[tauri::command]
pub fn set_autoplay_mode(
    mode: String,
    state: tauri::State<PlaybackPreferencesState>,
) -> Result<(), String> {
    let autoplay_mode = match mode.as_str() {
        "continue" => AutoplayMode::ContinueWithinSource,
        "track_only" => AutoplayMode::PlayTrackOnly,
        _ => return Err(format!("Invalid autoplay mode: {}", mode)),
    };
    state.set_autoplay_mode(autoplay_mode)
}
