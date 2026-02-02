//! Tauri commands for artist blacklist management

use tauri::State;

use crate::artist_blacklist::{BlacklistSettings, BlacklistState, BlacklistedArtist};

/// Get all blacklisted artists
#[tauri::command]
pub fn get_artist_blacklist(state: State<BlacklistState>) -> Result<Vec<BlacklistedArtist>, String> {
    state.get_all()
}

/// Add an artist to the blacklist
#[tauri::command]
pub fn add_to_artist_blacklist(
    artist_id: u64,
    artist_name: String,
    notes: Option<String>,
    state: State<BlacklistState>,
) -> Result<(), String> {
    state.add(artist_id, &artist_name, notes.as_deref())
}

/// Remove an artist from the blacklist
#[tauri::command]
pub fn remove_from_artist_blacklist(
    artist_id: u64,
    state: State<BlacklistState>,
) -> Result<(), String> {
    state.remove(artist_id)
}

/// Check if an artist is blacklisted
#[tauri::command]
pub fn is_artist_blacklisted(artist_id: u64, state: State<BlacklistState>) -> Result<bool, String> {
    Ok(state.is_blacklisted(artist_id))
}

/// Set the blacklist enabled state
#[tauri::command]
pub fn set_blacklist_enabled(enabled: bool, state: State<BlacklistState>) -> Result<(), String> {
    state.set_enabled(enabled)
}

/// Check if the blacklist feature is enabled
#[tauri::command]
pub fn is_blacklist_enabled(state: State<BlacklistState>) -> Result<bool, String> {
    Ok(state.is_enabled())
}

/// Get blacklist settings
#[tauri::command]
pub fn get_blacklist_settings(state: State<BlacklistState>) -> Result<BlacklistSettings, String> {
    state.get_settings()
}

/// Get the count of blacklisted artists
#[tauri::command]
pub fn get_blacklist_count(state: State<BlacklistState>) -> Result<usize, String> {
    Ok(state.count())
}

/// Clear all blacklisted artists
#[tauri::command]
pub fn clear_artist_blacklist(state: State<BlacklistState>) -> Result<(), String> {
    state.clear_all()
}
