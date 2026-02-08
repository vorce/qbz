//! Factory reset command
//!
//! Deletes all application data, cache, and config directories.
//! This is the nuclear option for unrecoverable states.

use tauri::State;

use crate::api_cache::ApiCacheState;
use crate::artist_blacklist::BlacklistState;
use crate::artist_vectors::ArtistVectorStoreState;
use crate::config::{
    audio_settings::AudioSettingsState,
    download_settings::DownloadSettingsState,
    favorites_cache::FavoritesCacheState,
    favorites_preferences::FavoritesPreferencesState,
    legal_settings::LegalSettingsState,
    playback_preferences::PlaybackPreferencesState,
    remote_control_settings::{AllowedOriginsState, RemoteControlSettingsState},
    subscription_state::SubscriptionStateState,
    tray_settings::TraySettingsState,
};
use crate::library::commands::LibraryState;
use crate::listenbrainz::ListenBrainzSharedState;
use crate::lyrics::LyricsState;
use crate::musicbrainz::MusicBrainzSharedState;
use crate::offline::OfflineState;
use crate::offline_cache::OfflineCacheState;
use crate::reco_store::RecoState;
use crate::session_store::SessionStoreState;
use crate::updates::UpdatesState;
use crate::user_data::UserDataPaths;
use crate::AppState;

use super::user_session::teardown_type_alias_state;

/// Delete all application data, cache, and config.
///
/// Steps:
/// 1. Stop playback
/// 2. Tear down all per-user stores (close DB connections)
/// 3. Clear credentials from keyring
/// 4. Remove ~/.local/share/qbz/
/// 5. Remove ~/.cache/qbz/
/// 6. Remove ~/.config/qbz/
#[tauri::command]
pub async fn factory_reset(
    app_state: State<'_, AppState>,
    user_paths: State<'_, UserDataPaths>,
    session_store: State<'_, SessionStoreState>,
    favorites_cache: State<'_, FavoritesCacheState>,
    subscription_state: State<'_, SubscriptionStateState>,
    playback_prefs: State<'_, PlaybackPreferencesState>,
    favorites_prefs: State<'_, FavoritesPreferencesState>,
    download_settings: State<'_, DownloadSettingsState>,
    audio_settings: State<'_, AudioSettingsState>,
    tray_settings: State<'_, TraySettingsState>,
    remote_control_settings: State<'_, RemoteControlSettingsState>,
    allowed_origins: State<'_, AllowedOriginsState>,
    legal_settings: State<'_, LegalSettingsState>,
    updates: State<'_, UpdatesState>,
    library: State<'_, LibraryState>,
    reco: State<'_, RecoState>,
    api_cache: State<'_, ApiCacheState>,
    artist_vectors: State<'_, ArtistVectorStoreState>,
    blacklist: State<'_, BlacklistState>,
    offline: State<'_, OfflineState>,
    offline_cache: State<'_, OfflineCacheState>,
    lyrics: State<'_, LyricsState>,
    musicbrainz: State<'_, MusicBrainzSharedState>,
    listenbrainz: State<'_, ListenBrainzSharedState>,
) -> Result<(), String> {
    log::warn!("FACTORY RESET: Starting — all application data will be deleted");

    // 1. Stop playback
    let _ = app_state.player.stop();
    app_state.media_controls.set_stopped();

    // 2. Tear down all per-user stores (close DB connections before deleting files)
    session_store.teardown();
    let _ = favorites_cache.teardown();
    let _ = playback_prefs.teardown();
    let _ = favorites_prefs.teardown();
    let _ = audio_settings.teardown();
    let _ = tray_settings.teardown();
    let _ = remote_control_settings.teardown();
    let _ = allowed_origins.teardown();
    updates.teardown();
    library.teardown().await;
    reco.teardown().await;
    api_cache.teardown().await;
    artist_vectors.teardown().await;
    blacklist.teardown();
    offline.teardown();
    offline_cache.teardown().await;
    lyrics.teardown().await;
    musicbrainz.teardown().await;
    listenbrainz.teardown().await;
    teardown_type_alias_state(&*subscription_state);
    teardown_type_alias_state(&*download_settings);
    teardown_type_alias_state(&*legal_settings);

    // Clear the active user
    user_paths.clear_user();
    UserDataPaths::clear_last_user_id();

    // 3. Clear credentials from keyring
    if let Err(e) = crate::credentials::clear_qobuz_credentials() {
        log::error!("FACTORY RESET: Failed to clear credentials: {}", e);
        // Continue anyway — deleting the dirs is more important
    }

    // 4. Remove data directory (~/.local/share/qbz/)
    if let Ok(data_dir) = UserDataPaths::global_data_dir() {
        if data_dir.exists() {
            log::info!("FACTORY RESET: Removing {}", data_dir.display());
            if let Err(e) = std::fs::remove_dir_all(&data_dir) {
                log::error!("FACTORY RESET: Failed to remove data dir: {}", e);
            }
        }
    }

    // 5. Remove cache directory (~/.cache/qbz/)
    if let Ok(cache_dir) = UserDataPaths::global_cache_dir() {
        if cache_dir.exists() {
            log::info!("FACTORY RESET: Removing {}", cache_dir.display());
            if let Err(e) = std::fs::remove_dir_all(&cache_dir) {
                log::error!("FACTORY RESET: Failed to remove cache dir: {}", e);
            }
        }
    }

    // 6. Remove config directory (~/.config/qbz/)
    if let Some(config_dir) = dirs::config_dir().map(|d| d.join("qbz")) {
        if config_dir.exists() {
            log::info!("FACTORY RESET: Removing {}", config_dir.display());
            if let Err(e) = std::fs::remove_dir_all(&config_dir) {
                log::error!("FACTORY RESET: Failed to remove config dir: {}", e);
            }
        }
    }

    log::warn!("FACTORY RESET: Complete — all application data deleted");
    Ok(())
}
