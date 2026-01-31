use tauri::command;
use crate::visualizer;

/// Enable or disable the audio visualizer
#[command]
pub fn set_visualizer_enabled(enabled: bool) {
    visualizer::set_visualizer_enabled(enabled);
}

/// Check if visualizer is enabled
#[command]
pub fn get_visualizer_enabled() -> bool {
    if let Ok(viz) = visualizer::VISUALIZER.lock() {
        viz.is_enabled()
    } else {
        false
    }
}
