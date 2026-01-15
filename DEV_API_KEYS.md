# Development with API Keys

## Problem

The API keys in `.env` are embedded at **compile-time** using Rust's `option_env!()` macro, not at runtime. This means the `.env` file must be loaded into environment variables **before** compilation.

## Solution for Local Development

Use the custom script that loads `.env` before building:

```bash
npm run dev:tauri
```

This script:
1. Sources the `.env` file into the shell environment
2. Runs `npm run tauri dev` with those variables available during compilation

## Alternative: Manual Export

If you prefer to run `npm run tauri dev` directly, export the variables first:

```bash
# Load .env into current shell
set -a
source .env
set +a

# Now run dev
npm run tauri dev
```

## Why This is Needed

- **CI/CD builds**: Variables are injected via GitHub Secrets → environment → `option_env!()` at compile time
- **Local development**: Variables must be in shell environment before `cargo build` runs
- The `dotenvy::dotenv()` in `lib.rs` only loads variables at **runtime**, but `option_env!()` reads them at **compile time**

## Embedded Keys

The following keys are embedded at compile time:
- `LAST_FM_API_KEY` / `LAST_FM_API_SHARED_SECRET`
- `DISCOGS_API_CLIENT_KEY` / `DISCOGS_API_CLIENT_SECRET`
- `SPOTIFY_API_CLIENT_ID` / `SPOTIFY_API_CLIENT_SECRET`
- `TIDAL_API_CLIENT_ID` / `TIDAL_API_CLIENT_SECRET`
