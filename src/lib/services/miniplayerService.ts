/**
 * MiniPlayer Window Management Service
 *
 * Handles creating, toggling, and controlling the MiniPlayer window.
 */

import { WebviewWindow } from '@tauri-apps/api/webviewWindow';

let miniplayerWindow: WebviewWindow | null = null;

/**
 * Create the MiniPlayer window
 */
export async function createMiniPlayer(): Promise<void> {
  // Check if window already exists
  try {
    const existing = await WebviewWindow.getByLabel('miniplayer');
    if (existing) {
      await existing.show();
      await existing.setFocus();
      return;
    }
  } catch {
    // Window doesn't exist, proceed to create
  }

  try {
    miniplayerWindow = new WebviewWindow('miniplayer', {
      url: '/miniplayer',
      title: 'QBZ MiniPlayer',
      width: 340,
      height: 90,
      minWidth: 300,
      minHeight: 80,
      maxHeight: 120,
      decorations: false,
      transparent: true,
      alwaysOnTop: true,
      resizable: false,
      skipTaskbar: true,
      // Position at top-right of screen
      x: 100,
      y: 100,
    });

    miniplayerWindow.once('tauri://created', () => {
      console.log('[MiniPlayer] Window created successfully');
    });

    miniplayerWindow.once('tauri://error', (e) => {
      console.error('[MiniPlayer] Failed to create window:', e);
    });
  } catch (err) {
    console.error('[MiniPlayer] Error creating window:', err);
  }
}

/**
 * Toggle MiniPlayer visibility
 */
export async function toggleMiniPlayer(): Promise<void> {
  try {
    const window = await WebviewWindow.getByLabel('miniplayer');
    if (!window) {
      await createMiniPlayer();
      return;
    }

    const isVisible = await window.isVisible();
    if (isVisible) {
      await window.hide();
    } else {
      await window.show();
      await window.setFocus();
    }
  } catch (err) {
    console.error('[MiniPlayer] Error toggling:', err);
    // Window might not exist, try creating it
    await createMiniPlayer();
  }
}

/**
 * Show the MiniPlayer window
 */
export async function showMiniPlayer(): Promise<void> {
  try {
    const window = await WebviewWindow.getByLabel('miniplayer');
    if (!window) {
      await createMiniPlayer();
      return;
    }
    await window.show();
    await window.setFocus();
  } catch {
    await createMiniPlayer();
  }
}

/**
 * Hide the MiniPlayer window
 */
export async function hideMiniPlayer(): Promise<void> {
  try {
    const window = await WebviewWindow.getByLabel('miniplayer');
    if (window) {
      await window.hide();
    }
  } catch (err) {
    console.error('[MiniPlayer] Error hiding:', err);
  }
}

/**
 * Close the MiniPlayer window
 */
export async function closeMiniPlayer(): Promise<void> {
  try {
    const window = await WebviewWindow.getByLabel('miniplayer');
    if (window) {
      await window.close();
      miniplayerWindow = null;
    }
  } catch (err) {
    console.error('[MiniPlayer] Error closing:', err);
  }
}

/**
 * Check if MiniPlayer is currently visible
 */
export async function isMiniPlayerVisible(): Promise<boolean> {
  try {
    const window = await WebviewWindow.getByLabel('miniplayer');
    if (!window) return false;
    return await window.isVisible();
  } catch {
    return false;
  }
}

/**
 * Set MiniPlayer always on top
 */
export async function setMiniPlayerAlwaysOnTop(alwaysOnTop: boolean): Promise<void> {
  try {
    const window = await WebviewWindow.getByLabel('miniplayer');
    if (window) {
      await window.setAlwaysOnTop(alwaysOnTop);
    }
  } catch (err) {
    console.error('[MiniPlayer] Error setting always on top:', err);
  }
}
