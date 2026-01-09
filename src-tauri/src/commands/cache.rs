//! Cache management commands

use tauri::State;

use crate::cache::CacheStats;
use crate::AppState;

/// Get cache statistics
#[tauri::command]
pub fn get_cache_stats(state: State<'_, AppState>) -> CacheStats {
    log::info!("Command: get_cache_stats");
    state.audio_cache.stats()
}

/// Clear the audio cache
#[tauri::command]
pub fn clear_cache(state: State<'_, AppState>) -> Result<(), String> {
    log::info!("Command: clear_cache");
    state.audio_cache.clear();
    Ok(())
}
