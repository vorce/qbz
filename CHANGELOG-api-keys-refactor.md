# API Keys Refactor - Changelog

## Overview

This branch removes all embedded API credentials from the QBZ binary to achieve Flathub compliance and eliminate security/technical debt. API keys are now managed server-side through a Cloudflare Workers proxy.

**Branch:** `feature/api-keys-refactor`
**Target:** `main`
**Impact:** Security, Privacy, Flathub Compliance

---

## 🔐 Security & Compliance

### Removed Embedded API Keys
- ❌ Removed Last.fm API secret from binary
- ❌ Removed Spotify client credentials from binary
- ❌ Removed Tidal client credentials from binary
- ❌ Removed Discogs API credentials from binary

### New Architecture
- ✅ Cloudflare Workers proxy handles all API authentication
- ✅ Client only knows proxy URL (no secrets exposed)
- ✅ Rate limiting and User-Agent validation server-side
- ✅ Ready for Flathub submission

---

## 🚀 New Features

### 1. Discogs Artwork Selection UI
**Enhanced album artwork management with Discogs integration**

- 🎨 **Visual artwork selector** in album edit modal
- 🔍 **Smart search**: Uses catalog number if available, falls back to artist + album
- 🖼️ **Multi-release support**: Shows images from top 2 release variants
- 📄 **Carousel navigation**: Browse 4 images per page with prev/next controls
- 📋 **Release metadata**: Each image shows release title and year
- ✨ **Automatic download**: Select image → Save → automatically cached locally

**User Benefits:**
- Audiophiles with catalog numbers get precise results
- See which release variant each artwork comes from
- Easy to find and set correct album covers

### 2. Catalog Number Support
**Precise music metadata for audiophile libraries**

- 🏷️ **Reads catalog number** from audio file tags (CATALOGNUMBER field)
- 💾 **Stores in database** for all tracks and albums
- 🎯 **Prioritizes in Discogs search** for accurate results
- 👁️ **Displays in UI** on album detail view (e.g., "Cat# MFSL 1-395")

**User Benefits:**
- Professional/audiophile collections with catalog numbers get better search results
- Can identify specific pressings and editions
- More accurate metadata overall

### 3. URI Scheme Registration
**Deep linking support for OAuth flows**

- 🔗 **Registered `qbz://` URI scheme** for OAuth callbacks
- 🌐 **Enables web-based authentication** flows
- 🔄 **Future-proof** for additional integrations

---

## 🔧 Technical Changes

### Cloudflare Workers Proxy
**Location:** `/home/blitzkriegfc/Personal/qbz/qbz-api-proxy`

#### Endpoints Implemented

**Last.fm**
- `POST /lastfm/auth.getMobileSession` - Mobile session authentication
- `GET /lastfm/*` - All Last.fm API methods (read-only)
- `POST /lastfm/*` - Write operations (scrobbles, love, etc.)
- Automatic signature generation with API secret

**Spotify**
- `POST /spotify/token` - OAuth token exchange
- `POST /spotify/refresh` - Token refresh
- Client credentials managed server-side

**Tidal**
- `POST /tidal/token` - OAuth token exchange
- `POST /tidal/refresh` - Token refresh
- Client credentials managed server-side

**Discogs**
- `GET /discogs/search` - Database search
- `GET /discogs/release/{id}` - Full release details with all images
- `GET /discogs/image` - Authenticated image download proxy

#### Security Features
- Rate limiting (configurable per client IP)
- User-Agent validation (rejects non-QBZ clients)
- CORS headers for web compatibility
- Request logging for monitoring

### Rust Backend Changes

#### Last.fm Client (`src-tauri/src/lastfm/client.rs`)
- Removed `api_secret` field and signature generation
- All requests now go through proxy
- Simplified authentication flow
- Maintains same public API for backward compatibility

#### Playlist Import (`src-tauri/src/playlist_import/`)
- Spotify import uses proxy for OAuth
- Tidal import uses proxy for OAuth
- No client credentials needed in binary

#### Discogs Integration (`src-tauri/src/discogs/mod.rs`)
- New `search_artwork_options()` method with catalog number support
- New `get_release_details()` for full image sets
- Intelligent image interleaving (4 from release #1, 4 from #2, 2 from others)
- Duplicate filtering by URL
- All requests through proxy (no credentials in binary)

#### Database Schema (`src-tauri/src/library/database.rs`)
- Added `catalog_number` column to `local_tracks` table
- Migration runs automatically on first launch
- Included in album aggregation queries

#### Metadata Extraction (`src-tauri/src/library/metadata.rs`)
- Reads `CatalogNumber` tag using lofty
- Extracts from FLAC, MP3, ALAC, etc.

### Frontend Changes

#### LocalLibraryView Component
- New Discogs artwork selector UI
- Carousel navigation for image options
- Release title and year display
- Catalog number display in album details
- Improved button consistency

---

## 📊 Database Migrations

### Migration: Add catalog_number Column
```sql
ALTER TABLE local_tracks ADD COLUMN catalog_number TEXT;
```

**Impact:**
- Runs automatically on app startup
- Non-destructive (only adds column)
- Existing data unaffected
- New scans will populate catalog numbers

---

## 🧪 Testing Notes

### What Was Tested
- ✅ Last.fm authentication and scrobbling through proxy
- ✅ Spotify playlist import with OAuth through proxy
- ✅ Tidal playlist import with OAuth through proxy
- ✅ Discogs artwork search with catalog numbers
- ✅ Multi-release image fetching and display
- ✅ Carousel navigation with 4+ images
- ✅ Artwork download and caching
- ✅ Database migration (catalog_number)

### Known Issues
- ⚠️ **429 Rate Limits**: Discogs API occasionally returns "Too Many Requests"
  - **Impact**: User must retry after a few seconds
  - **Frequency**: Occasional, especially when fetching detailed release info
  - **Severity**: Low (transient error, retry works)
  - **Mitigation**: Proxy uses rate limiting, but Discogs has strict limits
  - **Future Fix**: Could add request caching or debouncing

---

## 🚨 Breaking Changes

### For Developers
- `LastFmClient::new()` no longer requires `api_secret` parameter
- `LastFmClient::with_credentials()` signature changed (removed `api_secret`)
- Cloudflare Workers proxy must be deployed for app to function
- Environment variables needed for proxy deployment (see `.dev.vars.example`)

### For Users
**None** - Changes are transparent to end users. All existing functionality works the same.

---

## 🌐 Deployment Requirements

### Cloudflare Workers
The proxy must be deployed to Cloudflare Workers with these secrets:

```bash
# Last.fm
LAST_FM_API_KEY=your_key_here
LAST_FM_API_SHARED_SECRET=your_secret_here

# Spotify
SPOTIFY_API_CLIENT_ID=your_client_id_here
SPOTIFY_API_CLIENT_SECRET=your_client_secret_here

# Tidal
TIDAL_API_CLIENT_ID=your_client_id_here
TIDAL_API_CLIENT_SECRET=your_client_secret_here

# Discogs
DISCOGS_API_CLIENT_KEY=your_key_here
DISCOGS_API_CLIENT_SECRET=your_secret_here
```

**Deployment:**
```bash
cd qbz-api-proxy
npm install
wrangler deploy
```

**URL:** `https://qbz-api-proxy.blitzkriegfc.workers.dev`

---

## 📝 Migration Guide

### Merging to Main

1. **Ensure proxy is deployed** and responding
2. **Test all integrations** post-merge:
   - Last.fm login and scrobbling
   - Spotify playlist import
   - Tidal playlist import
   - Discogs artwork fetching
3. **Database migration runs automatically** on first launch
4. **No user action required** - changes are transparent

### Rollback Plan
If issues arise, revert commits and redeploy previous version. Database migration is additive only (safe to rollback).

---

## 🎯 Future Enhancements

### Potential Improvements
- [ ] **Request caching** in proxy to reduce 429 errors
- [ ] **Artwork preview** before download
- [ ] **Batch artwork fetching** for multiple albums
- [ ] **MusicBrainz integration** using same proxy pattern
- [ ] **User-provided API keys** as optional override

---

## 📄 Files Changed

### New Files
- `qbz-api-proxy/` - Complete Cloudflare Workers proxy project
- `CHANGELOG-api-keys-refactor.md` - This file

### Modified Files
**Rust Backend:**
- `src-tauri/src/lastfm/client.rs`
- `src-tauri/src/lastfm/commands.rs`
- `src-tauri/src/playlist_import/spotify.rs`
- `src-tauri/src/playlist_import/tidal.rs`
- `src-tauri/src/discogs/mod.rs`
- `src-tauri/src/library/models.rs`
- `src-tauri/src/library/database.rs`
- `src-tauri/src/library/metadata.rs`
- `src-tauri/src/library/cue_parser.rs`
- `src-tauri/src/library/commands.rs`
- `src-tauri/src/lib.rs`

**Frontend:**
- `src/lib/components/views/LocalLibraryView.svelte`

---

## 👥 Credits

- **Architecture Design**: API proxy pattern for credential management
- **Discogs Integration**: Multi-release artwork selection with metadata
- **Catalog Number Support**: Audiophile-grade metadata handling
- **UI/UX**: Carousel navigation and visual artwork selector

---

## 📅 Timeline

- **Started**: API keys refactor planning
- **Completed**: Full integration with all services migrated to proxy
- **Ready for Merge**: Pending main branch audio configuration updates
- **Target Merge**: After main stabilizes

---

## ✅ Pre-Merge Checklist

- [x] All API keys removed from binary
- [x] Cloudflare Workers proxy deployed and tested
- [x] Last.fm integration working through proxy
- [x] Spotify integration working through proxy
- [x] Tidal integration working through proxy
- [x] Discogs integration working through proxy
- [x] Database migration tested
- [x] UI updates complete and tested
- [x] No breaking changes for end users
- [x] Documentation complete (this CHANGELOG)
- [ ] Wait for main branch audio config changes to stabilize
- [ ] Final integration test after merge to main
- [ ] Update app version number
- [ ] Tag release

---

**End of Changelog**
