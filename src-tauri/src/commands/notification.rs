//! System notification commands

use notify_rust::Notification;

/// Show a track change notification
#[tauri::command]
pub fn show_track_notification(
    title: String,
    artist: String,
    album: String,
    _artwork_url: Option<String>,
) -> Result<(), String> {
    log::info!("Command: show_track_notification - {} by {}", title, artist);

    let body = if album.is_empty() {
        artist.clone()
    } else {
        format!("{} • {}", artist, album)
    };

    Notification::new()
        .summary(&title)
        .body(&body)
        .appname("QBZ")
        .timeout(3000) // 3 seconds
        .show()
        .map_err(|e| format!("Failed to show notification: {}", e))?;

    Ok(())
}

/// Show a generic notification
#[tauri::command]
pub fn show_notification(
    title: String,
    body: Option<String>,
) -> Result<(), String> {
    log::info!("Command: show_notification - {}", title);

    let mut notification = Notification::new();
    notification.summary(&title).appname("QBZ").timeout(3000);

    if let Some(body_text) = body {
        notification.body(&body_text);
    }

    notification
        .show()
        .map_err(|e| format!("Failed to show notification: {}", e))?;

    Ok(())
}
