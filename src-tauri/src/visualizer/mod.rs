use rustfft::{FftPlanner, num_complex::Complex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use rodio::Source;

/// Number of frequency bins to output (matches frontend bars)
const OUTPUT_BINS: usize = 32;

/// FFT window size (power of 2 for efficiency)
const FFT_SIZE: usize = 1024;

/// How often to compute FFT (in samples) - roughly 20-30fps at 44.1kHz
const FFT_INTERVAL: usize = 2048;

/// Visualizer state shared between audio thread and emitter
pub struct VisualizerState {
    /// Ring buffer for incoming samples
    sample_buffer: VecDeque<f32>,
    /// FFT planner (reusable)
    fft_planner: FftPlanner<f32>,
    /// Scratch buffer for FFT input
    fft_input: Vec<Complex<f32>>,
    /// Scratch buffer for FFT output
    fft_output: Vec<Complex<f32>>,
    /// Current frequency magnitudes (0.0 to 1.0)
    frequency_bins: [f32; OUTPUT_BINS],
    /// Smoothed bins for display (with decay)
    smoothed_bins: [f32; OUTPUT_BINS],
    /// Sample counter for FFT interval
    sample_count: usize,
    /// Current sample rate
    sample_rate: u32,
    /// Number of channels
    channels: u16,
}

impl VisualizerState {
    pub fn new() -> Self {
        Self {
            sample_buffer: VecDeque::with_capacity(FFT_SIZE * 2),
            fft_planner: FftPlanner::new(),
            fft_input: vec![Complex::new(0.0, 0.0); FFT_SIZE],
            fft_output: vec![Complex::new(0.0, 0.0); FFT_SIZE],
            frequency_bins: [0.0; OUTPUT_BINS],
            smoothed_bins: [0.0; OUTPUT_BINS],
            sample_count: 0,
            sample_rate: 44100,
            channels: 2,
        }
    }

    /// Update sample rate when track changes
    pub fn set_format(&mut self, sample_rate: u32, channels: u16) {
        self.sample_rate = sample_rate;
        self.channels = channels;
        // Clear buffer on format change
        self.sample_buffer.clear();
        self.sample_count = 0;
    }

    /// Push samples into the buffer (called from audio thread)
    /// Returns true if FFT should be computed
    pub fn push_samples(&mut self, samples: &[i16]) -> bool {
        // Convert i16 to f32 and mix to mono if stereo
        let channels = self.channels as usize;

        for chunk in samples.chunks(channels) {
            // Average channels to mono
            let mono: f32 = chunk.iter()
                .map(|&s| s as f32 / 32768.0)
                .sum::<f32>() / channels as f32;

            self.sample_buffer.push_back(mono);

            // Keep buffer bounded
            if self.sample_buffer.len() > FFT_SIZE * 4 {
                self.sample_buffer.pop_front();
            }
        }

        self.sample_count += samples.len() / channels;

        // Check if we should compute FFT
        if self.sample_count >= FFT_INTERVAL && self.sample_buffer.len() >= FFT_SIZE {
            self.sample_count = 0;
            true
        } else {
            false
        }
    }

    /// Compute FFT and update frequency bins
    pub fn compute_fft(&mut self) {
        if self.sample_buffer.len() < FFT_SIZE {
            return;
        }

        // Copy samples to FFT input with Hann window
        let start = self.sample_buffer.len().saturating_sub(FFT_SIZE);
        for (i, sample) in self.sample_buffer.iter().skip(start).take(FFT_SIZE).enumerate() {
            // Hann window to reduce spectral leakage
            let window = 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / (FFT_SIZE - 1) as f32).cos());
            self.fft_input[i] = Complex::new(sample * window, 0.0);
        }

        // Perform FFT
        let fft = self.fft_planner.plan_fft_forward(FFT_SIZE);
        self.fft_output.copy_from_slice(&self.fft_input);
        fft.process(&mut self.fft_output);

        // Convert to magnitude spectrum (only first half is meaningful)
        let useful_bins = FFT_SIZE / 2;

        // Map FFT bins to output bins (logarithmic scale for perceptual accuracy)
        // Lower frequencies get more bins, higher frequencies are grouped
        for i in 0..OUTPUT_BINS {
            // Logarithmic mapping: more resolution at low frequencies
            let start_bin = self.log_bin_start(i, useful_bins);
            let end_bin = self.log_bin_start(i + 1, useful_bins);

            // Average magnitude in this range
            let mut sum = 0.0f32;
            let mut count = 0;

            for bin in start_bin..end_bin.min(useful_bins) {
                let magnitude = self.fft_output[bin].norm();
                sum += magnitude;
                count += 1;
            }

            let avg = if count > 0 { sum / count as f32 } else { 0.0 };

            // Scale to 0-1 range (with some headroom)
            // The scaling factor is empirical - adjust for visual effect
            let scaled = (avg * 4.0 / FFT_SIZE as f32).min(1.0);

            self.frequency_bins[i] = scaled;
        }

        // Apply smoothing (fast attack, slow decay)
        for i in 0..OUTPUT_BINS {
            let current = self.frequency_bins[i];
            let smoothed = self.smoothed_bins[i];

            if current > smoothed {
                // Fast attack
                self.smoothed_bins[i] = smoothed + (current - smoothed) * 0.7;
            } else {
                // Slow decay
                self.smoothed_bins[i] = smoothed + (current - smoothed) * 0.15;
            }
        }
    }

    /// Logarithmic bin mapping for perceptual frequency distribution
    fn log_bin_start(&self, bin: usize, total_bins: usize) -> usize {
        // Map 0..OUTPUT_BINS to 0..total_bins logarithmically
        // This gives more resolution to lower frequencies
        let ratio = bin as f32 / OUTPUT_BINS as f32;
        let log_ratio = ratio.powf(2.0); // Quadratic gives good distribution
        (log_ratio * total_bins as f32) as usize
    }

    /// Get current smoothed bins for display
    pub fn get_bins(&self) -> [f32; OUTPUT_BINS] {
        self.smoothed_bins
    }

    /// Reset all bins (for pause/stop)
    pub fn reset(&mut self) {
        self.frequency_bins = [0.0; OUTPUT_BINS];
        self.smoothed_bins = [0.0; OUTPUT_BINS];
        self.sample_buffer.clear();
        self.sample_count = 0;
    }
}

/// Global visualizer instance
pub struct Visualizer {
    state: Arc<Mutex<VisualizerState>>,
    enabled: Arc<AtomicBool>,
    app_handle: Option<AppHandle>,
}

impl Visualizer {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(VisualizerState::new())),
            enabled: Arc::new(AtomicBool::new(false)),
            app_handle: None,
        }
    }

    /// Set the app handle for emitting events
    pub fn set_app_handle(&mut self, handle: AppHandle) {
        self.app_handle = Some(handle);
    }

    /// Enable/disable visualization
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::SeqCst);
        if !enabled {
            if let Ok(mut state) = self.state.lock() {
                state.reset();
            }
        }
    }

    /// Check if visualization is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst)
    }

    /// Update format when track changes
    pub fn set_format(&self, sample_rate: u32, channels: u16) {
        if let Ok(mut state) = self.state.lock() {
            state.set_format(sample_rate, channels);
        }
    }

    /// Push samples and emit if ready (non-blocking)
    /// Returns quickly - designed to be called from audio thread
    pub fn push_samples(&self, samples: &[i16]) {
        if !self.is_enabled() {
            return;
        }

        // Try to lock, but don't block audio thread
        if let Ok(mut state) = self.state.try_lock() {
            let should_emit = state.push_samples(samples);

            if should_emit {
                state.compute_fft();
                let bins = state.get_bins();
                drop(state); // Release lock before emitting

                self.emit_spectrum(bins);
            }
        }
        // If lock fails, skip this batch - visualization can afford to drop frames
    }

    /// Emit spectrum data to frontend
    fn emit_spectrum(&self, bins: [f32; OUTPUT_BINS]) {
        if let Some(ref handle) = self.app_handle {
            // Convert to Vec<f32> for JSON serialization
            let bins_vec: Vec<f32> = bins.to_vec();
            let _ = handle.emit("audio-spectrum", bins_vec);
        }
    }

    /// Reset visualization (for pause/stop)
    pub fn reset(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.reset();
        }
        // Emit zeros
        self.emit_spectrum([0.0; OUTPUT_BINS]);
    }

    /// Get a clone of the state Arc for sharing
    pub fn get_state(&self) -> Arc<Mutex<VisualizerState>> {
        self.state.clone()
    }

    /// Get a clone of the enabled flag for sharing
    pub fn get_enabled_flag(&self) -> Arc<AtomicBool> {
        self.enabled.clone()
    }
}

impl Default for Visualizer {
    fn default() -> Self {
        Self::new()
    }
}

// Global instance
lazy_static::lazy_static! {
    pub static ref VISUALIZER: Mutex<Visualizer> = Mutex::new(Visualizer::new());
}

/// Initialize visualizer with app handle
pub fn init_visualizer(handle: AppHandle) {
    if let Ok(mut viz) = VISUALIZER.lock() {
        viz.set_app_handle(handle);
        log::info!("[Visualizer] Initialized");
    }
}

/// Enable/disable visualizer
pub fn set_visualizer_enabled(enabled: bool) {
    if let Ok(viz) = VISUALIZER.lock() {
        viz.set_enabled(enabled);
        log::info!("[Visualizer] Enabled: {}", enabled);
    }
}

/// Update visualizer format
pub fn set_visualizer_format(sample_rate: u32, channels: u16) {
    if let Ok(viz) = VISUALIZER.lock() {
        viz.set_format(sample_rate, channels);
        log::debug!("[Visualizer] Format: {}Hz, {} channels", sample_rate, channels);
    }
}

/// Push samples to visualizer (called from audio thread)
pub fn push_visualizer_samples(samples: &[i16]) {
    if let Ok(viz) = VISUALIZER.try_lock() {
        viz.push_samples(samples);
    }
}

/// Reset visualizer
pub fn reset_visualizer() {
    if let Ok(viz) = VISUALIZER.lock() {
        viz.reset();
    }
}

/// A wrapper source that taps into audio samples for visualization
/// without modifying the audio flow. This is a zero-copy passthrough
/// that only observes samples.
pub struct VisualizerSource<S> {
    inner: S,
    buffer: Vec<i16>,
    buffer_threshold: usize,
    channels: u16,
    sample_rate: u32,
}

impl<S> VisualizerSource<S>
where
    S: Source<Item = i16>,
{
    /// Wrap a source with visualization tap
    pub fn new(inner: S) -> Self {
        let channels = inner.channels();
        let sample_rate = inner.sample_rate();

        // Set visualizer format
        set_visualizer_format(sample_rate, channels);

        // Buffer enough samples before sending to visualizer
        // ~50ms worth at a time to reduce overhead
        let buffer_threshold = (sample_rate as usize * channels as usize) / 20;

        Self {
            inner,
            buffer: Vec::with_capacity(buffer_threshold),
            buffer_threshold,
            channels,
            sample_rate,
        }
    }
}

impl<S> Iterator for VisualizerSource<S>
where
    S: Source<Item = i16>,
{
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.inner.next()?;

        // Collect samples in buffer
        self.buffer.push(sample);

        // When buffer is full, send to visualizer
        if self.buffer.len() >= self.buffer_threshold {
            push_visualizer_samples(&self.buffer);
            self.buffer.clear();
        }

        Some(sample)
    }
}

impl<S> Source for VisualizerSource<S>
where
    S: Source<Item = i16>,
{
    fn current_frame_len(&self) -> Option<usize> {
        self.inner.current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        self.inner.total_duration()
    }
}
