# HANDOFF: Audio Output Device Selection Implementation

## Status: COMPLETE - Backend and Frontend Wired

### What's Done

1. **Audio Settings Storage** (`src-tauri/src/config/audio_settings.rs`)
   - SQLite persistence for: `output_device`, `exclusive_mode`, `dac_passthrough`, `preferred_sample_rate`
   - Commands registered: `get_audio_settings`, `set_audio_output_device`, `set_audio_exclusive_mode`, `set_audio_dac_passthrough`, `set_audio_sample_rate`

2. **Device Listing** (`src-tauri/src/commands/playback.rs:295-328`)
   - `get_audio_devices()` - Lists available audio devices using rodio/cpal

3. **Player Device Selection** (`src-tauri/src/player/mod.rs`)
   - `Player::new(device_name: Option<String>)` - Accepts optional device name
   - Uses `OutputStream::try_from_device()` for specific device
   - Falls back to default device if specified device not found
   - Falls back to `try_default()` if device-specific stream fails

4. **AppState Integration** (`src-tauri/src/lib.rs`)
   - `AppState::with_device(device_name)` - Creates state with specific audio device
   - On app startup, reads saved device from audio settings
   - Passes saved device to Player initialization

5. **Frontend Integration** (`src/lib/components/views/SettingsView.svelte`)
   - `loadAudioSettings()` - Loads saved settings on mount
   - `handleOutputDeviceChange()` - Saves device to backend
   - `handleExclusiveModeChange()` - Saves exclusive mode to backend
   - `handleDacPassthroughChange()` - Saves DAC passthrough to backend
   - UI shows tooltip explaining changes take effect on next playback

### Testing Checklist

1. [ ] Select specific device in Settings - should save to database
2. [ ] Close and reopen app - device should persist
3. [ ] Play audio - should go to selected device
4. [ ] Toggle exclusive mode - verify behavior (Linux/ALSA specific)
5. [ ] Test with external DAC (user has DAC on Monday)

### Important Notes

- **Device changes take effect on app restart** - The Player is created once at startup with the saved device. Changing the device in settings saves it for the next session.
- **Exclusive mode on Linux** requires ALSA configuration, not just app-side changes
- **DAC passthrough** may need specific sample rate matching
- The app logs the audio device being used on startup (check terminal output)

### Future Enhancement: Runtime Device Switching

To switch devices without restart, would need to add an `AudioCommand::SelectDevice` variant and recreate the output stream in the audio thread. This is more complex because:
- Need to recreate OutputStream
- Need to handle any currently playing audio
- Need to preserve playback position if switching mid-track

This is optional and can be added later if needed.
