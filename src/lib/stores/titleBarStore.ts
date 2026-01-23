/**
 * Title Bar Settings Store
 *
 * Manages title bar visibility settings with localStorage persistence.
 *
 * Settings:
 * - useCustomTitleBar: Use QBZ custom title bar (default: true)
 * - hideTitleBar: Remove title bar completely for tiling WM users (default: false)
 */

import { getCurrentWindow } from '@tauri-apps/api/window';

const STORAGE_KEY_CUSTOM = 'qbz-use-custom-titlebar';
const STORAGE_KEY_HIDE = 'qbz-hide-titlebar';

// State
let useCustomTitleBar = true;
let hideTitleBar = false;

// Listeners
const listeners = new Set<() => void>();

function notifyListeners(): void {
  for (const listener of listeners) {
    listener();
  }
}

/**
 * Initialize the store from localStorage and apply settings
 */
export async function initTitleBarStore(): Promise<void> {
  try {
    const savedCustom = localStorage.getItem(STORAGE_KEY_CUSTOM);
    const savedHide = localStorage.getItem(STORAGE_KEY_HIDE);

    if (savedCustom !== null) {
      useCustomTitleBar = savedCustom === 'true';
    }
    if (savedHide !== null) {
      hideTitleBar = savedHide === 'true';
    }

    // Apply settings to window
    await applyTitleBarSettings();
  } catch (e) {
    console.error('[TitleBarStore] Failed to initialize:', e);
  }
}

/**
 * Subscribe to title bar state changes
 */
export function subscribe(listener: () => void): () => void {
  listeners.add(listener);
  listener();
  return () => listeners.delete(listener);
}

/**
 * Get current settings
 */
export function getUseCustomTitleBar(): boolean {
  return useCustomTitleBar;
}

export function getHideTitleBar(): boolean {
  return hideTitleBar;
}

/**
 * Determine if the custom title bar should be visible
 * - Hidden if hideTitleBar is true (tiling WM mode)
 * - Hidden if useCustomTitleBar is false (using system decorations)
 */
export function shouldShowTitleBar(): boolean {
  if (hideTitleBar) return false;
  return useCustomTitleBar;
}

/**
 * Get the title bar height for layout calculations
 * Returns 0 if title bar is hidden, 32 otherwise
 */
export function getTitleBarHeight(): number {
  return shouldShowTitleBar() ? 32 : 0;
}

/**
 * Apply current settings to the Tauri window
 */
async function applyTitleBarSettings(): Promise<void> {
  try {
    const appWindow = getCurrentWindow();

    if (hideTitleBar) {
      // No title bar at all - remove decorations
      await appWindow.setDecorations(false);
    } else if (useCustomTitleBar) {
      // Custom title bar - no system decorations
      await appWindow.setDecorations(false);
    } else {
      // System title bar - enable decorations
      await appWindow.setDecorations(true);
    }
  } catch (e) {
    console.error('[TitleBarStore] Failed to apply settings:', e);
  }
}

/**
 * Set whether to use custom title bar
 */
export async function setUseCustomTitleBar(value: boolean): Promise<void> {
  useCustomTitleBar = value;
  try {
    localStorage.setItem(STORAGE_KEY_CUSTOM, String(value));
    await applyTitleBarSettings();
  } catch (e) {
    console.error('[TitleBarStore] Failed to save custom titlebar setting:', e);
  }
  notifyListeners();
}

/**
 * Set whether to hide title bar completely
 */
export async function setHideTitleBar(value: boolean): Promise<void> {
  hideTitleBar = value;
  try {
    localStorage.setItem(STORAGE_KEY_HIDE, String(value));
    await applyTitleBarSettings();
  } catch (e) {
    console.error('[TitleBarStore] Failed to save hide titlebar setting:', e);
  }
  notifyListeners();
}

export interface TitleBarState {
  useCustomTitleBar: boolean;
  hideTitleBar: boolean;
  showTitleBar: boolean;
  titleBarHeight: number;
}

export function getTitleBarState(): TitleBarState {
  return {
    useCustomTitleBar,
    hideTitleBar,
    showTitleBar: shouldShowTitleBar(),
    titleBarHeight: getTitleBarHeight()
  };
}
