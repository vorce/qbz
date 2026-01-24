//! Playlist-related Tauri commands

use std::collections::HashMap;
use tauri::State;

use crate::api::QobuzClient;
use crate::api::models::{Playlist, SearchResultsPage};
use crate::library::commands::LibraryState;
use crate::library::compute_added_at_timestamp;
use crate::AppState;

/// Backfill the extra playlist metadata if needed. We need the metadata to support
/// additional functionality like sorting by "added at".
/// If there's any track in the playlist we don't have metadata for we will backfill it.
async fn maybe_backfill_playlist_metadata(
    playlist_id: u64,
    playlist: &mut Playlist,
    library_state: &State<'_, LibraryState>,
) -> Result<(), String> {
    let db = library_state.db.lock().await;
    let playlist_track_ids: Vec<u64> = playlist.tracks
        .as_ref()
        .map(|tracks| tracks.items.iter())
        .unwrap_or_else(|| [].iter())
        .filter_map(|track| track.playlist_track_id)
        .collect();

    let metadata = db
        .get_qobuz_track_metadata(playlist_id, &playlist_track_ids)
        .map_err(|e| format!("Failed to get track metadata: {}", e))?;

    let mut tracks_to_backfill = Vec::new();
    if let Some(ref mut tracks) = playlist.tracks {
        for (pos, track) in tracks.items.iter_mut().enumerate() {
            // Priority: API added_at (if present) > local database > position fallback
            // So when/if we get added_at info from Quboz we will use it automatically
            if track.added_at.is_none() {
                if let Some(ptid) = track.playlist_track_id {
                    if let Some((added_at, _)) = metadata.get(&ptid) {
                        track.added_at = Some(*added_at);
                    } else {
                        tracks_to_backfill.push((ptid, track.id, pos as i32));
                        track.added_at = Some(compute_added_at_timestamp(pos as i32, true));
                    }
                }
            };
        }
    }

    if !tracks_to_backfill.is_empty() {
        log::info!("Backfilling {} tracks for playlist {}", tracks_to_backfill.len(), playlist_id);
        db.track_qobuz_tracks_added(playlist_id, &tracks_to_backfill, true)
                 .map_err(|e| format!("Failed to backfill track metadata: {}", e))?;
    }

    Ok(())
}

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
    library_state: State<'_, LibraryState>,
) -> Result<Playlist, String> {
    log::info!("Command: get_playlist {}", playlist_id);

    let client = state.client.lock().await;
    let mut playlist = client
        .get_playlist(playlist_id)
        .await
        .map_err(|e| format!("Failed to get playlist: {}", e))?;

    drop(client); // Release the client lock before database operations
    maybe_backfill_playlist_metadata(playlist_id, &mut playlist, &library_state).await?;

    Ok(playlist)
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

/// Create a new playlist
#[tauri::command]
pub async fn create_playlist(
    name: String,
    description: Option<String>,
    is_public: Option<bool>,
    state: State<'_, AppState>,
) -> Result<Playlist, String> {
    log::info!("Command: create_playlist \"{}\"", name);

    let client = state.client.lock().await;
    client
        .create_playlist(&name, description.as_deref(), is_public.unwrap_or(false))
        .await
        .map_err(|e| format!("Failed to create playlist: {}", e))
}

/// Delete a playlist
#[tauri::command]
pub async fn delete_playlist(
    playlist_id: u64,
    state: State<'_, AppState>,
    library_state: State<'_, LibraryState>,
) -> Result<(), String> {
    log::info!("Command: delete_playlist {}", playlist_id);

    let client = state.client.lock().await;
    client
        .delete_playlist(playlist_id)
        .await
        .map_err(|e| format!("Failed to delete playlist: {}", e))?;

    // Clean up metadata for deleted playlist
    drop(client); // Release the client lock before acquiring database lock
    let db = library_state.db.lock().await;
    db.delete_all_qobuz_track_metadata(playlist_id)
        .map_err(|e| format!("Failed to delete playlist metadata: {}", e))?;

    Ok(())
}

async fn record_newly_added_tracks_metadata(
    playlist_id: u64,
    track_ids: &[u64],
    client: tokio::sync::MutexGuard<'_, QobuzClient>,
    library_state: State<'_, LibraryState>,
) -> Result<(), String> {
    let playlist = client
        .get_playlist(playlist_id)
        .await
        .map_err(|e| format!("Failed to fetch playlist after adding tracks: {}", e))?;

    // Find the newly added tracks and record their metadata
    if let Some(tracks) = playlist.tracks {
        let newly_added: Vec<(u64, u64, i32)> = tracks.items
            .iter()
            .filter_map(|track| {
                if track_ids.contains(&track.id) {
                    track.playlist_track_id.map(|ptid| {
                        (ptid, track.id, 1) // Position will be overwritten by track_qobuz_tracks_added 
                    })
                } else {
                    None
                }
            })
            .collect();

        if !newly_added.is_empty() {
            drop(client); // Release the client lock before acquiring database lock
            let db = library_state.db.lock().await;
            db.track_qobuz_tracks_added(playlist_id, &newly_added, false)
                .map_err(|e| format!("Failed to track newly added tracks: {}", e))?;
        }
    }

    Ok(())
}

/// Add tracks to a playlist
#[tauri::command]
pub async fn add_tracks_to_playlist(
    playlist_id: u64,
    track_ids: Vec<u64>,
    state: State<'_, AppState>,
    library_state: State<'_, LibraryState>,
) -> Result<(), String> {
    log::info!("Command: add_tracks_to_playlist {} ({} tracks)", playlist_id, track_ids.len());

    let client = state.client.lock().await;
    client
        .add_tracks_to_playlist(playlist_id, &track_ids)
        .await
        .map_err(|e| format!("Failed to add tracks to playlist: {}", e))?;

    record_newly_added_tracks_metadata(playlist_id, &track_ids, client, library_state).await?;

    Ok(())
}

/// Remove tracks from a playlist
#[tauri::command]
pub async fn remove_tracks_from_playlist(
    playlist_id: u64,
    playlist_track_ids: Vec<u64>,
    state: State<'_, AppState>,
    library_state: State<'_, LibraryState>,
) -> Result<(), String> {
    log::info!("Command: remove_tracks_from_playlist {} ({} tracks)", playlist_id, playlist_track_ids.len());

    let client = state.client.lock().await;
    client
        .remove_tracks_from_playlist(playlist_id, &playlist_track_ids)
        .await
        .map_err(|e| format!("Failed to remove tracks from playlist: {}", e))?;

    // Delete metadata for removed tracks
    drop(client); // Release the client lock before acquiring database lock
    let db = library_state.db.lock().await;
    db.delete_qobuz_track_metadata(playlist_id, &playlist_track_ids)
        .map_err(|e| format!("Failed to delete track metadata: {}", e))?;

    Ok(())
}

/// Update playlist metadata
#[tauri::command]
pub async fn update_playlist(
    playlist_id: u64,
    name: Option<String>,
    description: Option<String>,
    is_public: Option<bool>,
    state: State<'_, AppState>,
) -> Result<Playlist, String> {
    log::info!("Command: update_playlist {}", playlist_id);

    let client = state.client.lock().await;
    client
        .update_playlist(playlist_id, name.as_deref(), description.as_deref(), is_public)
        .await
        .map_err(|e| format!("Failed to update playlist: {}", e))
}

/// Get multiple tracks by their IDs
#[tauri::command]
pub async fn get_tracks_by_ids(
    track_ids: Vec<u64>,
    state: State<'_, AppState>,
) -> Result<Vec<crate::api::models::Track>, String> {
    log::info!("Command: get_tracks_by_ids ({} tracks)", track_ids.len());

    let client = state.client.lock().await;
    let mut tracks = Vec::new();

    for track_id in track_ids {
        match client.get_track(track_id).await {
            Ok(track) => tracks.push(track),
            Err(e) => {
                log::warn!("Failed to get track {}: {}", track_id, e);
                // Continue with other tracks even if one fails
            }
        }
    }

    Ok(tracks)
}
