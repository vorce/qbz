// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Use xdg-desktop-portal for file dialogs on Linux
    // This makes GTK apps use native file pickers (e.g., KDE's on Plasma)
    #[cfg(target_os = "linux")]
    std::env::set_var("GTK_USE_PORTAL", "1");

    // Prefer a writable TMPDIR to avoid GTK pixbuf cache crashes on some systems.
    #[cfg(target_os = "linux")]
    {
        if std::env::var_os("TMPDIR").is_none() {
            if let Some(cache_dir) = dirs::cache_dir() {
                let tmp_dir = cache_dir.join("qbz/tmp");
                if std::fs::create_dir_all(&tmp_dir).is_ok() {
                    std::env::set_var("TMPDIR", tmp_dir);
                }
            }
        }
    }

    qbz_nix_lib::run()
}
