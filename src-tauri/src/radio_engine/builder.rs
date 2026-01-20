use super::db::{RadioDb, RadioSeed, RadioSession};
use crate::api::{QobuzClient, Track, TracksContainer};

#[derive(Debug, Clone)]
pub struct BuildRadioOptions {
    pub rng_seed: u64,
    pub artist_spacing: u32,
    pub reseed_every: u32,

    pub seed_tracks_limit: u32,
}

impl Default for BuildRadioOptions {
    fn default() -> Self {
        Self {
            rng_seed: 0,
            artist_spacing: 5,
            reseed_every: 25,
            seed_tracks_limit: 150,
        }
    }
}

pub struct RadioPoolBuilder<'a> {
    pub db: &'a RadioDb,
    pub client: &'a QobuzClient,
    pub options: BuildRadioOptions,
}

impl<'a> RadioPoolBuilder<'a> {
    pub fn new(db: &'a RadioDb, client: &'a QobuzClient, options: BuildRadioOptions) -> Self {
        Self { db, client, options }
    }

    fn track_artist_id(track: &Track, fallback: u64) -> u64 {
        if let Some(performer) = track.performer.as_ref() {
            if performer.id != 0 {
                return performer.id;
            }
        }
        fallback
    }

    fn is_music_track(track: &Track) -> bool {
        track.streamable && track.duration > 0 && track.album.is_some()
    }

    fn derive_rng_seed(default_seed: u64, salt: u64) -> u64 {
        if default_seed != 0 {
            return default_seed;
        }
        (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0))
            ^ salt
    }

    pub async fn create_artist_radio(&self, artist_id: u64) -> Result<RadioSession, String> {
        let rng_seed = Self::derive_rng_seed(self.options.rng_seed, artist_id);
        let session = self.db.create_session(
            RadioSeed::Artist { artist_id },
            rng_seed,
            self.options.artist_spacing,
            self.options.reseed_every,
        )?;

        self.add_seed_artist_tracks(&session.id, artist_id).await?;

        Ok(session)
    }

    pub async fn create_track_radio(&self, track_id: u64, artist_id: u64) -> Result<RadioSession, String> {
        let rng_seed = Self::derive_rng_seed(self.options.rng_seed, track_id);
        let session = self.db.create_session(
            RadioSeed::Track { track_id, artist_id },
            rng_seed,
            self.options.artist_spacing,
            self.options.reseed_every,
        )?;

        let track = self
            .client
            .get_track(track_id)
            .await
            .map_err(|e| format!("Failed to fetch seed track: {}", e))?;

        let track_artist_id = Self::track_artist_id(&track, artist_id);
        if Self::is_music_track(&track) {
            self.db
                .insert_pool_track(&session.id, track.id, track_artist_id, "seed_track", 0)?;
        }

        self.add_seed_artist_tracks(&session.id, artist_id).await?;

        Ok(session)
    }

    async fn add_seed_artist_tracks(&self, session_id: &str, seed_artist_id: u64) -> Result<(), String> {
        let tracks: TracksContainer = self
            .client
            .get_artist_tracks(seed_artist_id, self.options.seed_tracks_limit, 0)
            .await
            .map_err(|e| format!("Failed to fetch seed artist tracks: {}", e))?;

        for t in tracks.items {
            if !Self::is_music_track(&t) {
                continue;
            }
            let artist_id = Self::track_artist_id(&t, seed_artist_id);
            self.db
                .insert_pool_track(session_id, t.id, artist_id, "seed_tracks", 0)?;
        }

        Ok(())
    }
}

