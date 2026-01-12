<p align="center">
  <img src="static/logo.png" alt="QBZ logo" width="180" />
</p>

<p align="center">
  <a href="https://github.com/vicrodh/qbz"><img src="https://img.shields.io/badge/github-vicrodh%2Fqbz-0b0b0b?style=flat&logo=github" alt="GitHub repo" /></a>
  <a href="https://github.com/vicrodh/qbz/releases"><img src="https://img.shields.io/github/v/release/vicrodh/qbz?style=flat" alt="Release" /></a>
  <a href="https://aur.archlinux.org/packages/qbz-bin"><img src="https://img.shields.io/aur/version/qbz-bin?style=flat&logo=archlinux" alt="AUR" /></a>
  <a href="https://github.com/vicrodh/qbz"><img src="https://img.shields.io/github/license/vicrodh/qbz?style=flat" alt="License" /></a>
  <a href="https://github.com/vicrodh/qbz"><img src="https://img.shields.io/badge/platform-Linux-0b0b0b?style=flat&logo=linux" alt="Platform" /></a>
</p>

# QBZ

QBZ is a free and open source (FOSS) Qobuz client for Linux with native, high-fidelity playback. It is a real desktop application, not a web wrapper, so it can use DAC passthrough, preserve hi-res sample rates end-to-end, and deliver bit-perfect audio.

## Why QBZ

Browsers cap audio output around 48 kHz, while Qobuz streams up to 192 kHz. QBZ uses a native playback pipeline with direct device control so your system and DAC receive the original resolution, with caching and system integrations that wrappers cannot provide.

## Installation

### Arch Linux (AUR)

```bash
# Using yay
yay -S qbz-bin

# Using paru
paru -S qbz-bin
```

### Flatpak

```bash
# Download from releases
flatpak install ./QBZ.flatpak
```

### AppImage / Tarball

Download the latest release from the [Releases](https://github.com/vicrodh/qbz/releases) page.

> **Note:** Pre-built binaries include all API integrations (Last.fm, Discogs, Spotify, Tidal) ready to use. If you build from source, you'll need to provide your own API keys.

## Screenshots

Coming soon.

## Features

### Streaming and Playback
- Qobuz authentication and full catalog search (albums, tracks, artists, playlists).
- Native decoding for FLAC, MP3, AAC, and ALAC with real-time playback state updates.
- Quality selection with automatic fallback across Qobuz tiers.
- Audio device enumeration and per-device output selection.
- DAC passthrough mode for bit-perfect playback.
- Gapless-ready playback pipeline with precise position tracking.

### Queue and Library
- Queue management with shuffle, repeat, and history navigation.
- In-memory audio cache with LRU eviction and next-track prefetching.
- Favorites and playlists from your Qobuz account.
- Local library backend: directory scanning, metadata extraction, CUE sheet parsing, and SQLite indexing.

### Playlist Import
- Import playlists from Spotify and Tidal into your Qobuz library.
- Automatic track matching with fuzzy search.
- Batch import with progress tracking.

### Network Casting
- Chromecast device discovery and streaming.
- DLNA/UPnP device discovery and streaming (AVTransport SOAP).
- Unified cast picker with protocol selection.
- Seamless playback handoff to network devices.

### Integrations
- MPRIS media controls and media key support on Linux.
- Desktop notifications for track changes.
- Last.fm scrobbling and now-playing updates.
- Discogs artwork fetching for local library.
- Shareable Qobuz URLs and universal SongLink links (Odesli).

### Interface
- Now playing, queue panel, and full-screen playback views.
- Focus mode for distraction-free listening.
- Mini player mode.
- Keyboard shortcuts for common actions.
- English and Spanish localization.

### Settings
- Audio device selection and quality preferences.
- API keys configuration for self-hosted builds.
- Theme and appearance options.

## Open Source

QBZ is MIT-licensed and fully open source. No telemetry, no lock-in, and no hidden services. Just a clean, transparent player built for Linux audio fans.

## Inspiration

QBZ draws inspiration from projects like qobuz-dl, and from the broader Linux audio community that values open tools and high-fidelity playback.

## Tech Stack

- **Desktop:** Rust + Tauri 2
- **Frontend:** SvelteKit 5 + TypeScript + Vite
- **Audio:** rodio + symphonia
- **Networking:** reqwest
- **Local library:** walkdir + lofty + rusqlite
- **Integrations:** souvlaki (MPRIS), notify-rust, SongLink (Odesli)
- **UX:** svelte-i18n, lucide-svelte

## Building from Source

### Prerequisites

- Rust (latest stable)
- Node.js 20+
- Linux with audio support (PipeWire, ALSA, or PulseAudio)
- System dependencies: `webkit2gtk-4.1`, `gtk3`, `alsa-lib`

### Setup

```bash
# Clone the repository
git clone https://github.com/vicrodh/qbz.git
cd qbz

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

### Environment Variables

When building from source, you need to provide your own API keys. Copy the example environment file:

```bash
cp .env.example .env
```

#### Last.fm Integration

1. Go to [Last.fm API Account](https://www.last.fm/api/account/create)
2. Create a new application
3. Add to `.env`:

```env
LAST_FM_API_KEY=your_api_key
LAST_FM_API_SHARED_SECRET=your_shared_secret
```

#### Discogs Integration (Local Library Artwork)

1. Go to [Discogs Developer Settings](https://www.discogs.com/settings/developers)
2. Create a new application
3. Add to `.env`:

```env
DISCOGS_API_CLIENT_KEY=your_consumer_key
DISCOGS_API_CLIENT_SECRET=your_consumer_secret
```

#### Spotify Integration (Playlist Import)

1. Go to [Spotify Developer Dashboard](https://developer.spotify.com/dashboard)
2. Create a new application
3. Add to `.env`:

```env
SPOTIFY_API_CLIENT_ID=your_client_id
SPOTIFY_API_CLIENT_SECRET=your_client_secret
```

#### Tidal Integration (Playlist Import)

1. Go to [Tidal Developer Portal](https://developer.tidal.com/)
2. Create a new application
3. Add to `.env`:

```env
TIDAL_API_CLIENT_ID=your_client_id
TIDAL_API_CLIENT_SECRET=your_client_secret
```

> **Note:** All integrations are optional. The application will work without them, but the corresponding features will be disabled.

## Project Structure

```
qbz/
├── src/                  # Frontend (SvelteKit)
├── src-tauri/
│   └── src/
│       ├── api/          # Qobuz API client
│       ├── player/       # Audio playback engine
│       ├── queue/        # Queue management
│       ├── cache/        # Audio cache and prefetch
│       ├── cast/         # Chromecast & DLNA casting
│       ├── library/      # Local library backend
│       ├── lastfm/       # Last.fm integration
│       ├── discogs/      # Discogs integration
│       ├── playlist_import/ # Spotify/Tidal import
│       ├── share/        # SongLink / share utilities
│       ├── media_controls/ # MPRIS integration
│       └── commands/     # Tauri IPC commands
├── packaging/
│   ├── aur/              # Arch Linux PKGBUILD
│   └── flatpak/          # Flatpak manifest
└── static/               # Static assets and logo
```

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

MIT
