//! Audio diagnostics commands for detecting actual hardware sample rate
//! and verifying effective bit depth of the audio pipeline.

use std::fs;

use tauri::Manager;
use crate::AppState;
use crate::audio::BitDepthResult;

/// Hardware audio status
#[derive(Debug, Clone, serde::Serialize)]
pub struct HardwareAudioStatus {
    /// Actual sample rate being used by hardware (Hz)
    pub hardware_sample_rate: Option<u32>,
    /// Audio format (e.g., "S32_LE")
    pub hardware_format: Option<String>,
    /// Whether hardware is currently active
    pub is_active: bool,
}

/// Get actual hardware audio status by reading /proc/asound
#[tauri::command]
pub fn get_hardware_audio_status() -> Result<HardwareAudioStatus, String> {
    // Read all hw_params files from /proc/asound
    let proc_pattern = "/proc/asound/card*/pcm*/sub*/hw_params";
    
    // Use glob to find all matching files
    let paths = glob::glob(proc_pattern)
        .map_err(|e| format!("Failed to glob hw_params: {}", e))?;
    
    for path in paths.flatten() {
        if let Ok(contents) = fs::read_to_string(&path) {
            // Parse the hw_params file
            let mut sample_rate: Option<u32> = None;
            let mut format: Option<String> = None;
            
            for line in contents.lines() {
                let line = line.trim();
                
                // Parse rate line: "rate: 192000 (192000/1)"
                if line.starts_with("rate:") {
                    if let Some(rate_str) = line.split_whitespace().nth(1) {
                        sample_rate = rate_str.parse().ok();
                    }
                }
                
                // Parse format line: "format: S32_LE"
                if line.starts_with("format:") {
                    if let Some(fmt) = line.split_whitespace().nth(1) {
                        format = Some(fmt.to_string());
                    }
                }
            }
            
            // If we found both, return it
            if sample_rate.is_some() {
                log::debug!(
                    "Hardware audio status: {}Hz, format: {:?}",
                    sample_rate.unwrap_or(0),
                    format
                );

                return Ok(HardwareAudioStatus {
                    hardware_sample_rate: sample_rate,
                    hardware_format: format,
                    is_active: true,
                });
            }
        }
    }
    
    // No active hardware found
    Ok(HardwareAudioStatus {
        hardware_sample_rate: None,
        hardware_format: None,
        is_active: false,
    })
}

/// Start bit-depth diagnostic capture on the currently playing audio.
/// Samples are analyzed on-the-fly with zero allocation (OR-mask of i32 values).
/// Call `stop_bitdepth_capture` after a few seconds to get the result.
#[tauri::command]
pub fn start_bitdepth_capture(
    app: tauri::AppHandle,
) -> Result<String, String> {
    let state = app.state::<AppState>();
    let sample_rate = state.player.state.get_sample_rate();
    if sample_rate == 0 {
        return Err("No audio playing — cannot start capture".to_string());
    }
    // Default to 2 channels (stereo) — SharedState doesn't track channels
    state.player.diagnostic.start_capture(sample_rate, 2);
    Ok(format!("Capture started at {}Hz", sample_rate))
}

/// Stop bit-depth capture and return analysis.
///
/// The `effective_bits` field is the key result:
/// - 24 → source data has 24-bit precision (f32 pipeline working)
/// - 16 → source data truncated to 16-bit (bug)
/// - Other values → intermediate precision or pipeline issue
#[tauri::command]
pub fn stop_bitdepth_capture(
    app: tauri::AppHandle,
) -> Result<BitDepthResult, String> {
    let state = app.state::<AppState>();
    Ok(state.player.diagnostic.stop_and_analyze())
}
