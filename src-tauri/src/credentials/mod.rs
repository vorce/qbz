//! Secure credential storage using system keyring
//!
//! Uses the system's secure credential storage:
//! - Linux: Secret Service (GNOME Keyring, KWallet via D-Bus)
//! - macOS: Keychain
//! - Windows: Credential Manager

use keyring::Entry;
use serde::{Deserialize, Serialize};

const SERVICE_NAME: &str = "qbz-nix";
const QOBUZ_CREDENTIALS_KEY: &str = "qobuz-credentials";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QobuzCredentials {
    pub email: String,
    pub password: String,
}

/// Save Qobuz credentials to system keyring
pub fn save_qobuz_credentials(email: &str, password: &str) -> Result<(), String> {
    let entry = Entry::new(SERVICE_NAME, QOBUZ_CREDENTIALS_KEY)
        .map_err(|e| format!("Failed to create keyring entry: {}", e))?;

    let credentials = QobuzCredentials {
        email: email.to_string(),
        password: password.to_string(),
    };

    let json = serde_json::to_string(&credentials)
        .map_err(|e| format!("Failed to serialize credentials: {}", e))?;

    entry
        .set_password(&json)
        .map_err(|e| format!("Failed to save to keyring: {}", e))?;

    log::info!("Qobuz credentials saved to system keyring");
    Ok(())
}

/// Load Qobuz credentials from system keyring
pub fn load_qobuz_credentials() -> Result<Option<QobuzCredentials>, String> {
    let entry = Entry::new(SERVICE_NAME, QOBUZ_CREDENTIALS_KEY)
        .map_err(|e| format!("Failed to create keyring entry: {}", e))?;

    match entry.get_password() {
        Ok(json) => {
            let credentials: QobuzCredentials = serde_json::from_str(&json)
                .map_err(|e| format!("Failed to parse credentials: {}", e))?;
            log::info!("Loaded Qobuz credentials from keyring for: {}", credentials.email);
            Ok(Some(credentials))
        }
        Err(keyring::Error::NoEntry) => {
            log::debug!("No saved Qobuz credentials found");
            Ok(None)
        }
        Err(e) => Err(format!("Failed to load from keyring: {}", e)),
    }
}

/// Check if credentials are saved
pub fn has_saved_credentials() -> bool {
    let entry = match Entry::new(SERVICE_NAME, QOBUZ_CREDENTIALS_KEY) {
        Ok(e) => e,
        Err(_) => return false,
    };

    entry.get_password().is_ok()
}

/// Clear saved Qobuz credentials
pub fn clear_qobuz_credentials() -> Result<(), String> {
    let entry = Entry::new(SERVICE_NAME, QOBUZ_CREDENTIALS_KEY)
        .map_err(|e| format!("Failed to create keyring entry: {}", e))?;

    match entry.delete_credential() {
        Ok(()) => {
            log::info!("Qobuz credentials cleared from keyring");
            Ok(())
        }
        Err(keyring::Error::NoEntry) => {
            // Already cleared, that's fine
            Ok(())
        }
        Err(e) => Err(format!("Failed to clear credentials: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credentials_roundtrip() {
        // Note: This test requires a working keyring service
        // Skip in CI environments
        if std::env::var("CI").is_ok() {
            return;
        }

        let email = "test@example.com";
        let password = "testpass123";

        // Save
        save_qobuz_credentials(email, password).expect("Failed to save");

        // Load
        let loaded = load_qobuz_credentials()
            .expect("Failed to load")
            .expect("No credentials found");

        assert_eq!(loaded.email, email);
        assert_eq!(loaded.password, password);

        // Clear
        clear_qobuz_credentials().expect("Failed to clear");

        // Verify cleared
        let after_clear = load_qobuz_credentials().expect("Failed to check");
        assert!(after_clear.is_none());
    }
}
