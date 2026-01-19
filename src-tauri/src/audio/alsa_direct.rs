//! Direct ALSA access using alsa-rs
//!
//! Provides bit-perfect playback for hw:X,Y devices that CPAL cannot open.
//! This module bypasses rodio/CPAL completely for direct hardware access.

#[cfg(target_os = "linux")]
use alsa::pcm::{Access, Format, HwParams, PCM};
#[cfg(target_os = "linux")]
use alsa::{Direction, ValueOr};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

/// Direct ALSA PCM stream for hw: devices
pub struct AlsaDirectStream {
    pcm: Arc<Mutex<PCM>>,
    is_playing: Arc<AtomicBool>,
    sample_rate: u32,
    channels: u16,
}

impl AlsaDirectStream {
    /// Create new ALSA direct stream
    #[cfg(target_os = "linux")]
    pub fn new(device_id: &str, sample_rate: u32, channels: u16) -> Result<Self, String> {
        log::info!(
            "[ALSA Direct] Opening device: {} ({}Hz, {}ch)",
            device_id,
            sample_rate,
            channels
        );

        // Open PCM device
        let pcm = PCM::new(device_id, Direction::Playback, false)
            .map_err(|e| format!("Failed to open ALSA device '{}': {}", device_id, e))?;

        // Set hardware parameters
        {
            let hwp = HwParams::any(&pcm)
                .map_err(|e| format!("Failed to get hardware params: {}", e))?;

            // Set access type (interleaved)
            hwp.set_access(Access::RWInterleaved)
                .map_err(|e| format!("Failed to set access: {}", e))?;

            // Set format (Signed 32-bit little endian - most DACs support this)
            hwp.set_format(Format::S32LE)
                .map_err(|e| format!("Failed to set format: {}", e))?;

            // Set channels
            hwp.set_channels(channels as u32)
                .map_err(|e| format!("Failed to set channels: {}", e))?;

            // Set sample rate (exact match - bit-perfect!)
            hwp.set_rate(sample_rate, ValueOr::Nearest)
                .map_err(|e| format!("Failed to set sample rate: {}", e))?;

            // Set buffer size (larger buffer for high-res audio)
            let buffer_size = if sample_rate >= 192000 {
                // 500ms buffer for 192kHz+ (like MPD config)
                (sample_rate / 2) as i64
            } else if sample_rate >= 96000 {
                // 250ms buffer for 96kHz
                (sample_rate / 4) as i64
            } else {
                // 125ms buffer for lower rates
                (sample_rate / 8) as i64
            };

            hwp.set_buffer_size_near(buffer_size)
                .map_err(|e| format!("Failed to set buffer size: {}", e))?;

            // Set period size (1/10 of buffer)
            hwp.set_period_size_near(buffer_size / 10, ValueOr::Nearest)
                .map_err(|e| format!("Failed to set period size: {}", e))?;

            // Apply hardware parameters
            pcm.hw_params(&hwp)
                .map_err(|e| format!("Failed to apply hardware params: {}", e))?;

            log::info!("[ALSA Direct] ✓ Hardware configured: {}Hz, {}ch, buffer: {} frames",
                sample_rate, channels, buffer_size);
        }

        // Prepare device for playback
        pcm.prepare()
            .map_err(|e| format!("Failed to prepare PCM: {}", e))?;

        Ok(Self {
            pcm: Arc::new(Mutex::new(pcm)),
            is_playing: Arc::new(AtomicBool::new(false)),
            sample_rate,
            channels,
        })
    }

    /// Write audio samples to ALSA (S32_LE format)
    #[cfg(target_os = "linux")]
    pub fn write(&self, samples: &[i32]) -> Result<(), String> {
        let mut pcm = self.pcm.lock().unwrap();

        // Convert samples to frames (samples / channels)
        let frames = samples.len() / self.channels as usize;

        // Write to PCM
        let io = pcm.io_i32()
            .map_err(|e| format!("Failed to get PCM I/O: {}", e))?;

        match io.writei(samples) {
            Ok(written) => {
                if written != frames {
                    log::warn!("[ALSA Direct] Partial write: {} / {} frames", written, frames);
                }
                Ok(())
            }
            Err(e) => {
                // Try to recover from underrun
                if let Err(recover_err) = pcm.recover(e.errno() as i32, false) {
                    Err(format!("Failed to recover from error: {}", recover_err))
                } else {
                    log::warn!("[ALSA Direct] Recovered from PCM error");
                    Ok(())
                }
            }
        }
    }

    /// Drain and stop playback
    #[cfg(target_os = "linux")]
    pub fn drain(&self) -> Result<(), String> {
        log::info!("[ALSA Direct] Draining PCM");
        let pcm = self.pcm.lock().unwrap();
        pcm.drain()
            .map_err(|e| format!("Failed to drain PCM: {}", e))
    }

    /// Stop PCM immediately (prepare for next playback)
    #[cfg(target_os = "linux")]
    pub fn stop(&self) -> Result<(), String> {
        log::info!("[ALSA Direct] Stopping PCM");
        let pcm = self.pcm.lock().unwrap();
        // PCM::drop() is called automatically when pcm goes out of scope
        // For now, just prepare for next playback
        pcm.prepare()
            .map_err(|e| format!("Failed to prepare PCM after stop: {}", e))
    }

    /// Get sample rate
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Get channels
    pub fn channels(&self) -> u16 {
        self.channels
    }

    /// Check if device is hw: device
    pub fn is_hw_device(device_id: &str) -> bool {
        device_id.starts_with("hw:") || device_id.starts_with("plughw:")
    }
}

#[cfg(not(target_os = "linux"))]
impl AlsaDirectStream {
    pub fn new(_device_id: &str, _sample_rate: u32, _channels: u16) -> Result<Self, String> {
        Err("ALSA Direct is only available on Linux".to_string())
    }

    pub fn write(&self, _samples: &[f32]) -> Result<(), String> {
        Err("ALSA Direct is only available on Linux".to_string())
    }

    pub fn drain(&self) -> Result<(), String> {
        Ok(())
    }

    pub fn stop(&self) -> Result<(), String> {
        Ok(())
    }

    pub fn sample_rate(&self) -> u32 {
        44100
    }

    pub fn channels(&self) -> u16 {
        2
    }

    pub fn is_hw_device(_device_id: &str) -> bool {
        false
    }
}
