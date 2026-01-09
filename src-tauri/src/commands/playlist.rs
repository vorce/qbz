//! Playlist-related Tauri commands

use tauri::State;

use crate::api::models::{Playlist, SearchResultsPage};
use crate::AppState;

/// Get user's playlists
#[tauri::command]
pub async fn get_user_playlists(
    state: State<'_, AppState>,
) -> Result<Vec<Playlist>, String> {
    log::info!("Command: get_user_playlists");

    let client = state.client.lock().await;
    client
        .get_user_playlists()
        .await
        .map_err(|e| format!("Failed to get user playlists: {}", e))
}

/// Get a specific playlist by ID
#[tauri::command]
pub async fn get_playlist(
    playlist_id: u64,
    state: State<'_, AppState>,
) -> Result<Playlist, String> {
    log::info!("Command: get_playlist {}", playlist_id);

    let client = state.client.lock().await;
    client
        .get_playlist(playlist_id)
        .await
        .map_err(|e| format!("Failed to get playlist: {}", e))
}

/// Search playlists
#[tauri::command]
pub async fn search_playlists(
    query: String,
    limit: Option<u32>,
    state: State<'_, AppState>,
) -> Result<SearchResultsPage<Playlist>, String> {
    log::info!("Command: search_playlists \"{}\"", query);

    let client = state.client.lock().await;
    client
        .search_playlists(&query, limit.unwrap_or(20))
        .await
        .map_err(|e| format!("Failed to search playlists: {}", e))
}
