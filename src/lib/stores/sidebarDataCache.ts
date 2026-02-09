/**
 * Sidebar Data Cache Store (SWR)
 *
 * Module-level cache for sidebar data (playlists, settings, local track counts).
 * Uses sessionStorage for persistence across navigation.
 *
 * Cache invalidation policy:
 * - Invalidate on real playlist changes: create, delete, track add/remove
 * - Do NOT invalidate on: sort order (local), play counts, UI state
 */

const SESSION_KEY = 'qbz-sidebar-cache';

export interface SidebarCacheData {
  playlists: unknown[];
  playlistSettings: unknown[];
  localTrackCounts: Record<string, number>;
  timestamp: number;
}

const FRESH_TTL_MS = 5 * 60 * 1000;   // 5 minutes
const MAX_TTL_MS = 60 * 60 * 1000;     // 60 minutes

export type SidebarCacheStatus = 'fresh' | 'stale' | 'empty';

let cache: SidebarCacheData | null = null;

function restoreFromSession(): void {
  try {
    const stored = sessionStorage.getItem(SESSION_KEY);
    if (stored) {
      cache = JSON.parse(stored);
    }
  } catch {
    // sessionStorage not available or corrupted
  }
}

function persistToSession(): void {
  if (!cache) {
    try { sessionStorage.removeItem(SESSION_KEY); } catch {}
    return;
  }
  try {
    sessionStorage.setItem(SESSION_KEY, JSON.stringify(cache));
  } catch {
    // sessionStorage full or not available
  }
}

// Initialize from sessionStorage
restoreFromSession();

export function getSidebarCacheStatus(): SidebarCacheStatus {
  if (!cache) return 'empty';

  const age = Date.now() - cache.timestamp;
  if (age > MAX_TTL_MS) return 'empty';
  if (age <= FRESH_TTL_MS) return 'fresh';
  return 'stale';
}

export function getSidebarCache(): SidebarCacheData | null {
  return cache;
}

export function setSidebarCache(data: Omit<SidebarCacheData, 'timestamp'>): void {
  cache = {
    ...data,
    timestamp: Date.now(),
  };
  persistToSession();
}

export function clearSidebarCache(): void {
  cache = null;
  persistToSession();
}
