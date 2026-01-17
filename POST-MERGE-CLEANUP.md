# Post-Merge Cleanup: Remove API Keys UI

**Context:** The API Keys section in Settings UI is non-functional legacy code. All API requests now go through Cloudflare Workers proxy. User-provided credentials are accepted but ignored.

**When to do this:** After merging to main and audio configuration changes are stable.

---

## Files to Delete Completely

### 1. Delete API Keys Store Module
```bash
rm src-tauri/src/api_keys.rs
```

**Why:** Entire module is unused. User credentials are collected but never used.

---

## Files to Modify

### 2. src-tauri/src/lib.rs

**Remove these imports:**
```rust
// Line ~8
mod api_keys;
```

**Remove state initialization:**
```rust
// Around line 162
let api_keys_state = api_keys::create_api_keys_state();
```

**Remove from .manage() call:**
```rust
// Around line 400-420 in the .manage() chain
.manage(api_keys_state)
```

**Remove command registrations:**
```rust
// Around line 530-539 in invoke_handler![...]
api_keys::set_spotify_credentials,
api_keys::clear_spotify_credentials,
api_keys::has_spotify_user_credentials,
api_keys::set_tidal_credentials,
api_keys::clear_tidal_credentials,
api_keys::has_tidal_user_credentials,
api_keys::set_discogs_credentials,
api_keys::clear_discogs_credentials,
api_keys::has_discogs_user_credentials,
api_keys::get_embedded_credentials_status,
```

---

### 3. src-tauri/src/commands/playlist_import.rs

**Remove import:**
```rust
// Line 5
use crate::api_keys::ApiKeysState;
```

**Remove state parameter from commands:**

**In `playlist_import_preview()` (line 13-42):**

Before:
```rust
pub async fn playlist_import_preview(
    url: String,
    api_keys: State<'_, ApiKeysState>,
) -> Result<ImportPlaylist, String> {
    // Get user-provided credentials from state
    let keys = api_keys.lock().await;
    let spotify_creds = if keys.spotify.is_set() {
        Some(ProviderCredentials { ... })
    } else {
        None
    };
    let tidal_creds = if keys.tidal.is_set() {
        Some(ProviderCredentials { ... })
    } else {
        None
    };
    drop(keys);

    preview_public_playlist(&url, spotify_creds, tidal_creds)
        .await
        .map_err(|e| e.to_string())
}
```

After:
```rust
pub async fn playlist_import_preview(
    url: String,
) -> Result<ImportPlaylist, String> {
    preview_public_playlist(&url, None, None)
        .await
        .map_err(|e| e.to_string())
}
```

**In `playlist_import_execute()` (line 45-90):**

Remove the same pattern:
- Remove `api_keys: State<'_, ApiKeysState>` parameter
- Remove credential extraction logic (lines 54-68)
- Pass `None, None` instead of `spotify_creds, tidal_creds`

---

### 4. src-tauri/src/library/commands.rs

**Remove import:**
```rust
// Line 10
use crate::api_keys::ApiKeysState;
```

**Remove state parameter from these commands:**

1. **`library_fetch_missing_artwork`** (line ~1168)
2. **`discogs_search_artist`** (line ~1187)
3. **`library_import_musicbrainz_tags`** (line ~1240)
4. **`library_set_artist_image`** (line ~1312)

**Pattern to follow for each:**

Before:
```rust
pub async fn command_name(
    // ... other parameters
    api_keys: State<'_, ApiKeysState>,
) -> Result<T, String> {
    // Get Discogs client
    let keys = api_keys.lock().await;
    let discogs_client = if keys.discogs.is_set() {
        DiscogsClient::with_user_credentials(
            keys.discogs.client_id.clone(),
            keys.discogs.client_secret.clone(),
        )
    } else {
        DiscogsClient::new()
    };
    drop(keys);

    // ... rest of function
}
```

After:
```rust
pub async fn command_name(
    // ... other parameters
) -> Result<T, String> {
    // Get Discogs client (proxy handles credentials)
    let discogs_client = DiscogsClient::new();

    // ... rest of function
}
```

**Specific locations:**
- Line ~1175-1182: Remove credential logic in `library_fetch_missing_artwork`
- Line ~1193-1200: Remove credential logic in `discogs_search_artist`
- Line ~1256-1263: Remove credential logic in `library_import_musicbrainz_tags`
- Line ~1320-1327: Remove credential logic in `library_set_artist_image`

---

### 5. src-tauri/src/playlist_import/providers/mod.rs

**Simplify `fetch_playlist` function:**

Before (line ~43-61):
```rust
pub async fn fetch_playlist(
    kind: ProviderKind,
    spotify_creds: Option<ProviderCredentials>,
    tidal_creds: Option<ProviderCredentials>,
) -> Result<ImportPlaylist, PlaylistImportError> {
    match kind {
        ProviderKind::Spotify { playlist_id } => {
            spotify::fetch_playlist(&playlist_id, spotify_creds).await
        }
        ProviderKind::AppleMusic { storefront, playlist_id } => {
            apple::fetch_playlist(&storefront, &playlist_id).await
        }
        ProviderKind::Tidal { playlist_id } => {
            tidal::fetch_playlist(&playlist_id, tidal_creds).await
        }
        ProviderKind::Deezer { playlist_id } => deezer::fetch_playlist(&playlist_id).await,
    }
}
```

After:
```rust
pub async fn fetch_playlist(
    kind: ProviderKind,
) -> Result<ImportPlaylist, PlaylistImportError> {
    match kind {
        ProviderKind::Spotify { playlist_id } => {
            spotify::fetch_playlist(&playlist_id).await
        }
        ProviderKind::AppleMusic { storefront, playlist_id } => {
            apple::fetch_playlist(&storefront, &playlist_id).await
        }
        ProviderKind::Tidal { playlist_id } => {
            tidal::fetch_playlist(&playlist_id).await
        }
        ProviderKind::Deezer { playlist_id } => deezer::fetch_playlist(&playlist_id).await,
    }
}
```

---

### 6. src-tauri/src/playlist_import/providers/spotify.rs

**Simplify `fetch_playlist` signature:**

Before (line ~39-50):
```rust
pub async fn fetch_playlist(
    playlist_id: &str,
    user_creds: Option<ProviderCredentials>,
) -> Result<ImportPlaylist, PlaylistImportError> {
    if let Ok(token) = get_app_token(user_creds).await {
        if let Ok(playlist) = fetch_playlist_with_token(playlist_id, &token).await {
            return Ok(playlist);
        }
    }
    fetch_playlist_from_embed(playlist_id).await
}
```

After:
```rust
pub async fn fetch_playlist(
    playlist_id: &str,
) -> Result<ImportPlaylist, PlaylistImportError> {
    if let Ok(token) = get_app_token().await {
        if let Ok(playlist) = fetch_playlist_with_token(playlist_id, &token).await {
            return Ok(playlist);
        }
    }
    fetch_playlist_from_embed(playlist_id).await
}
```

**Simplify `get_app_token` signature:**

Before (line ~252-280):
```rust
async fn get_app_token(_user_creds: Option<ProviderCredentials>) -> Result<String, PlaylistImportError> {
    // Proxy handles credentials - user_creds ignored (compatibility)
    // ...
}
```

After:
```rust
async fn get_app_token() -> Result<String, PlaylistImportError> {
    // Proxy handles credentials
    // ...
}
```

---

### 7. src-tauri/src/playlist_import/providers/tidal.rs

**Same changes as Spotify:**

1. Remove `user_creds` parameter from `fetch_playlist()` signature
2. Remove `_user_creds` parameter from `get_app_token()` signature
3. Update function calls to not pass credentials

---

### 8. src-tauri/src/discogs/mod.rs

**Simplify `DiscogsClient`:**

**Remove this method entirely (line ~51-69):**
```rust
pub fn with_user_credentials(
    _user_key: Option<String>,
    _user_secret: Option<String>,
) -> Self {
    // ...entire method...
}
```

**Update documentation comment (line ~46):**

Before:
```rust
/// Create a new Discogs client (proxy handles credentials)
pub fn new() -> Self {
    Self::with_user_credentials(None, None)
}

/// Create a new Discogs client - compatibility method (proxy handles credentials)
pub fn with_user_credentials(...) -> Self { ... }
```

After:
```rust
/// Create a new Discogs client (proxy handles credentials)
pub fn new() -> Self {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static("QBZ/1.0.0"),
    );

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .default_headers(headers)
        .build()
        .expect("Failed to create HTTP client");

    Self { client }
}
```

---

### 9. src/lib/components/views/SettingsView.svelte

**Remove from sections array:**
```svelte
// Around line 121
{ id: 'api-keys', label: 'API Keys' },
```

**Remove state variables (line 262-271):**
```svelte
// API Keys section state
let apiKeysExpanded = $state(false);
let spotifyClientId = $state('');
let spotifyClientSecret = $state('');
let tidalClientId = $state('');
let tidalClientSecret = $state('');
let discogsConsumerKey = $state('');
let discogsConsumerSecret = $state('');
let embeddedStatus = $state({ spotify: false, tidal: false, discogs: false, lastfm: false });
let apiKeysSaving = $state(false);
```

**Remove helper functions (find and delete):**
```svelte
function hasAnyUserCredentials() { ... }
async function loadApiKeysStatus() { ... }
async function saveSpotifyCredentials() { ... }
async function clearSpotifyCredentials() { ... }
async function saveTidalCredentials() { ... }
async function clearTidalCredentials() { ... }
async function saveDiscogsCredentials() { ... }
async function clearDiscogsCredentials() { ... }
```

**Remove from onMount:**
```svelte
// Around line 300-320 in onMount()
// Load embedded credentials status
loadApiKeysStatus();
```

**Remove entire section element (line ~1494-1650):**
```svelte
<!-- API Keys Section (collapsible) -->
<section class="section api-keys-section" bind:this={apiKeysSection}>
  <!-- ...entire section... -->
</section>
```

**Remove CSS styles (line ~2112-2300+):**
```css
/* API Keys section styles */
.api-keys-section { ... }
.section-title-btn { ... }
.keys-badge { ... }
.api-keys-info { ... }
.api-key-group { ... }
.api-key-header { ... }
.api-key-title { ... }
.status-badge { ... }
.api-key-desc { ... }
.api-key-fields { ... }
.input-group { ... }
.input-group label { ... }
.input-group input { ... }
.api-key-actions { ... }
```

---

### 10. src/lib/i18n/locales/en.json

**Remove API Keys translations:**
```json
// Find and remove entire "apiKeys" object under "settings.integrations"
"apiKeys": "API Keys",
"apiKeysDesc": "Optional custom API credentials...",
// ... etc
```

---

### 11. src/lib/i18n/locales/es.json

**Remove API Keys translations:**
```json
// Find and remove entire "apiKeys" object under "settings.integrations"
"apiKeys": "Claves API",
"apiKeysDesc": "Credenciales API personalizadas opcionales...",
// ... etc
```

---

## Verification Steps

After cleanup:

1. **Build check:**
   ```bash
   cargo check
   ```

2. **Search for remnants:**
   ```bash
   # Should return nothing
   grep -r "api_keys" src-tauri/src/ --include="*.rs"
   grep -r "ApiKeysState" src-tauri/src/ --include="*.rs"
   grep -r "apiKeys" src/lib/ --include="*.svelte" --include="*.ts"
   ```

3. **Test that app builds:**
   ```bash
   npm run tauri build
   ```

4. **Verify Settings UI:**
   - Open Settings
   - "API Keys" section should be gone
   - Audio settings should be intact
   - No console errors

---

## Size Reduction Estimate

**Code removed:**
- `api_keys.rs`: ~175 lines
- Command registrations: ~10 lines
- State management: ~5 lines
- Settings UI: ~150 lines HTML + ~200 lines CSS
- Helper functions: ~120 lines
- Translations: ~40 lines
- Import cleanup: ~15 lines

**Total: ~715 lines of unused code removed**

---

## Git Commit Message Template

```
Remove unused API Keys UI and infrastructure

All API requests now go through Cloudflare Workers proxy.
User-provided credentials were collected but never used.

Removed:
- src-tauri/src/api_keys.rs (entire module)
- API Keys section from Settings UI
- ApiKeysState and related commands
- User credential parameters from providers
- Simplified DiscogsClient (removed with_user_credentials)
- Cleaned up imports and translations

This cleanup reduces codebase by ~715 lines and removes
confusing UI that promised functionality that didn't work.

See: POST-MERGE-CLEANUP.md for complete removal instructions
```

---

## Notes

- **Do this AFTER** main branch audio configuration changes are merged
- **Do this in a separate commit** for clean history
- **Test thoroughly** after removal
- If proxy goes down in future, this can be reverted and implemented properly
- Keep this file in repo as documentation of what was removed and why

---

**Created:** January 17, 2026
**Status:** Pending execution post-merge
**Estimated time:** 30-45 minutes
