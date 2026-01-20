import { writable } from 'svelte/store';

/**
 * Store to track download settings changes
 * Increments whenever download settings are updated (download root or show_in_library)
 * Components can subscribe to this to refresh when settings change
 */
export const downloadSettingsVersion = writable(0);

/**
 * Call this whenever download settings are updated
 * This triggers a refresh in components that subscribe to downloadSettingsVersion
 */
export function notifyDownloadSettingsChanged() {
  downloadSettingsVersion.update(n => n + 1);
}
