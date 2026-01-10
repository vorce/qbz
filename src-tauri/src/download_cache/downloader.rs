//! Download logic for caching tracks to disk

use std::io::Write;
use std::path::Path;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

use super::{DownloadProgress, DownloadStatus};

/// Downloader handles fetching audio files and saving them to disk
pub struct Downloader {
    client: reqwest::Client,
}

impl Downloader {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(300)) // 5 minute timeout for large files
            .connect_timeout(Duration::from_secs(15))
            .build()
            .expect("Failed to create HTTP client");

        Self { client }
    }

    /// Download a file to disk with progress updates
    pub async fn download_to_file(
        &self,
        url: &str,
        dest_path: &Path,
        track_id: u64,
        app_handle: Option<&AppHandle>,
    ) -> Result<u64, String> {
        log::info!("Downloading track {} to {:?}", track_id, dest_path);

        // Create parent directories if needed
        if let Some(parent) = dest_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        // Start the download
        let response = self
            .client
            .get(url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await
            .map_err(|e| format!("Failed to start download: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("HTTP error: {}", response.status()));
        }

        let total_size = response.content_length();
        log::info!(
            "Download started for track {}, total size: {:?} bytes",
            track_id,
            total_size
        );

        // Create temp file for downloading
        let temp_path = dest_path.with_extension("tmp");
        let mut file = std::fs::File::create(&temp_path)
            .map_err(|e| format!("Failed to create temp file: {}", e))?;

        let mut downloaded: u64 = 0;
        let mut last_progress: u8 = 0;

        // Stream the response body
        let mut stream = response.bytes_stream();
        use futures_util::StreamExt;

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| format!("Download error: {}", e))?;

            file.write_all(&chunk)
                .map_err(|e| format!("Failed to write chunk: {}", e))?;

            downloaded += chunk.len() as u64;

            // Calculate progress
            let progress = if let Some(total) = total_size {
                ((downloaded as f64 / total as f64) * 100.0) as u8
            } else {
                // If we don't know total size, report bytes downloaded
                0
            };

            // Emit progress event every 2% change
            if progress != last_progress && (progress - last_progress >= 2 || progress == 100) {
                last_progress = progress;

                if let Some(app) = app_handle {
                    let _ = app.emit(
                        "download:progress",
                        DownloadProgress {
                            track_id,
                            progress_percent: progress,
                            bytes_downloaded: downloaded,
                            total_bytes: total_size,
                            status: DownloadStatus::Downloading,
                        },
                    );
                }

                log::debug!(
                    "Download progress for track {}: {}% ({}/{:?} bytes)",
                    track_id,
                    progress,
                    downloaded,
                    total_size
                );
            }
        }

        // Ensure all data is written
        file.flush()
            .map_err(|e| format!("Failed to flush file: {}", e))?;
        drop(file);

        // Move temp file to final destination
        std::fs::rename(&temp_path, dest_path)
            .map_err(|e| format!("Failed to move temp file: {}", e))?;

        log::info!(
            "Download complete for track {}: {} bytes",
            track_id,
            downloaded
        );

        Ok(downloaded)
    }

    /// Download to memory (for smaller files or streaming)
    pub async fn download_to_memory(&self, url: &str) -> Result<Vec<u8>, String> {
        let response = self
            .client
            .get(url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await
            .map_err(|e| format!("Failed to fetch: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("HTTP error: {}", response.status()));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to read bytes: {}", e))?;

        Ok(bytes.to_vec())
    }
}

impl Default for Downloader {
    fn default() -> Self {
        Self::new()
    }
}

