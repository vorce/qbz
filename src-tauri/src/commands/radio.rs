//! Radio Engine Tauri commands
//!
//! Creates and manages radio sessions for infinite playback discovery

use tauri::State;

use crate::api::Track;
use crate::playback_context::{ContextData, PlaybackContext};
use crate::queue::QueueTrack;
use crate::radio_engine::{BuildRadioOptions, RadioEngine, RadioPoolBuilder};
use crate::AppState;

/// Create an artist radio session
///
/// This creates a new radio session based on an artist and populates the queue
/// with initial tracks from the radio engine.
#[tauri::command]
pub async fn create_artist_radio(
    artist_id: u64,
    artist_name: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    log::info!("[Radio] Creating artist radio for: {} (ID: {})", artist_name, artist_id);

    // Initialize radio engine if not exists
    let radio_db = crate::radio_engine::db::RadioDb::open_default()?;
    let radio_engine = RadioEngine::new(radio_db);

    // Get QobuzClient
    let client = state.client.lock().await;

    // Create radio session
    let builder = RadioPoolBuilder::new(
        radio_engine.db(),
        &client,
        BuildRadioOptions::default(),
    );

    let session = builder.create_artist_radio(artist_id).await?;
    let session_id = session.id.clone();

    log::info!("[Radio] Artist radio session created: {}", session_id);

    // Generate initial tracks
    let mut tracks = Vec::new();
    for _ in 0..15 {
        match radio_engine.next_track(&session_id) {
            Ok(radio_track) => {
                // Fetch full track details from Qobuz
                match client.get_track(radio_track.track_id).await {
                    Ok(track) => {
                        tracks.push(track);
                    }
                    Err(e) => {
                        log::warn!("[Radio] Failed to fetch track {}: {}", radio_track.track_id, e);
                    }
                }
            }
            Err(e) => {
                log::warn!("[Radio] Failed to get next radio track: {}", e);
                break;
            }
        }
    }

    if tracks.is_empty() {
        return Err("Failed to generate any radio tracks".to_string());
    }

    log::info!("[Radio] Generated {} initial tracks", tracks.len());

    // Convert to QueueTrack format
    let queue_tracks: Vec<QueueTrack> = tracks.iter().map(track_to_queue_track).collect();

    // Set the queue
    state.queue.set_queue(queue_tracks, Some(0));

    // Set playback context to radio
    let context = PlaybackContext {
        context_type: "radio".to_string(),
        id: session_id.clone(),
        label: artist_name,
        data: ContextData {
            tracks: tracks.clone(),
            start_index: Some(0),
        },
    };
    state.context.set_context(context);

    log::info!("[Radio] Artist radio ready: {}", session_id);

    Ok(session_id)
}

/// Create a track radio session
///
/// This creates a new radio session based on a specific track and populates the queue
/// with initial tracks from the radio engine.
#[tauri::command]
pub async fn create_track_radio(
    track_id: u64,
    track_name: String,
    artist_id: u64,
    state: State<'_, AppState>,
) -> Result<String, String> {
    log::info!(
        "[Radio] Creating track radio for: {} (Track ID: {}, Artist ID: {})",
        track_name,
        track_id,
        artist_id
    );

    // Initialize radio engine
    let radio_db = crate::radio_engine::db::RadioDb::open_default()?;
    let radio_engine = RadioEngine::new(radio_db);

    // Get QobuzClient
    let client = state.client.lock().await;

    // Create radio session
    let builder = RadioPoolBuilder::new(
        radio_engine.db(),
        &client,
        BuildRadioOptions::default(),
    );

    let session = builder.create_track_radio(track_id, artist_id).await?;
    let session_id = session.id.clone();

    log::info!("[Radio] Track radio session created: {}", session_id);

    // Generate initial tracks
    let mut tracks = Vec::new();
    for _ in 0..15 {
        match radio_engine.next_track(&session_id) {
            Ok(radio_track) => {
                // Fetch full track details from Qobuz
                match client.get_track(radio_track.track_id).await {
                    Ok(track) => {
                        tracks.push(track);
                    }
                    Err(e) => {
                        log::warn!("[Radio] Failed to fetch track {}: {}", radio_track.track_id, e);
                    }
                }
            }
            Err(e) => {
                log::warn!("[Radio] Failed to get next radio track: {}", e);
                break;
            }
        }
    }

    if tracks.is_empty() {
        return Err("Failed to generate any radio tracks".to_string());
    }

    log::info!("[Radio] Generated {} initial tracks", tracks.len());

    // Convert to QueueTrack format
    let queue_tracks: Vec<QueueTrack> = tracks.iter().map(track_to_queue_track).collect();

    // Set the queue
    state.queue.set_queue(queue_tracks, Some(0));

    // Set playback context to radio
    let context = PlaybackContext {
        context_type: "radio".to_string(),
        id: session_id.clone(),
        label: track_name,
        data: ContextData {
            tracks: tracks.clone(),
            start_index: Some(0),
        },
    };
    state.context.set_context(context);

    log::info!("[Radio] Track radio ready: {}", session_id);

    Ok(session_id)
}

/// Convert API Track to QueueTrack
fn track_to_queue_track(track: &Track) -> QueueTrack {
    let artwork_url = track
        .album
        .as_ref()
        .and_then(|a| a.image.as_ref())
        .and_then(|img| img.large.clone());

    let artist = track
        .performer
        .as_ref()
        .map(|p| p.name.clone())
        .unwrap_or_else(|| "Unknown Artist".to_string());

    let album = track
        .album
        .as_ref()
        .map(|a| a.title.clone())
        .unwrap_or_else(|| "Unknown Album".to_string());

    QueueTrack {
        id: track.id,
        title: track.title.clone(),
        artist,
        album,
        duration_secs: track.duration,
        artwork_url,
        hires: track.hires.unwrap_or(false),
        bit_depth: track.maximum_bit_depth,
        sample_rate: track.maximum_sampling_rate,
        is_local: false,
    }
}
