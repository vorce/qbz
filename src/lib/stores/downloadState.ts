import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

export type DownloadStatus = 'none' | 'queued' | 'downloading' | 'ready' | 'failed';

export interface DownloadInfo {
  status: DownloadStatus;
  progress: number;
  error?: string;
}

export interface CachedTrackInfo {
  trackId: number;
  title: string;
  artist: string;
  album?: string;
  albumId?: string;
  durationSecs: number;
  fileSizeBytes: number;
  quality: string;
  bitDepth?: number;
  sampleRate?: number;
  status: DownloadStatus;
  progressPercent: number;
  errorMessage?: string;
  createdAt: string;
  lastAccessedAt: string;
}

export interface DownloadCacheStats {
  totalTracks: number;
  readyTracks: number;
  downloadingTracks: number;
  failedTracks: number;
  totalSizeBytes: number;
  limitBytes?: number;
  cachePath: string;
}

// Track download states by track ID
const downloadStates = new Map<number, DownloadInfo>();

// Listeners for state changes
const listeners = new Set<() => void>();

// Event unsubscribe functions
let unlisteners: UnlistenFn[] = [];

export function getDownloadState(trackId: number): DownloadInfo {
  return downloadStates.get(trackId) || { status: 'none', progress: 0 };
}

export function setDownloadState(trackId: number, info: DownloadInfo): void {
  downloadStates.set(trackId, info);
  notifyListeners();
}

export function subscribe(listener: () => void): () => void {
  listeners.add(listener);
  return () => listeners.delete(listener);
}

function notifyListeners(): void {
  for (const listener of listeners) {
    listener();
  }
}

// Initialize download states from backend
export async function initDownloadStates(): Promise<void> {
  try {
    const tracks = await invoke<CachedTrackInfo[]>('get_downloaded_tracks');
    for (const track of tracks) {
      downloadStates.set(track.trackId, {
        status: track.status,
        progress: track.progressPercent,
        error: track.errorMessage,
      });
    }
    notifyListeners();
  } catch (err) {
    console.error('Failed to init download states:', err);
  }
}

// Start listening for download events
export async function startDownloadEventListeners(): Promise<void> {
  try {
    const unlistenStarted = await listen<{ trackId: number }>('download:started', (event) => {
      console.log('Download started:', event.payload.trackId);
      setDownloadState(event.payload.trackId, { status: 'downloading', progress: 0 });
    });

    const unlistenProgress = await listen<{
      trackId: number;
      progressPercent: number;
      bytesDownloaded: number;
      totalBytes?: number;
      status: string;
    }>('download:progress', (event) => {
      const { trackId, progressPercent } = event.payload;
      setDownloadState(trackId, { status: 'downloading', progress: progressPercent });
    });

    const unlistenCompleted = await listen<{ trackId: number; size: number }>('download:completed', (event) => {
      console.log('Download completed:', event.payload.trackId);
      setDownloadState(event.payload.trackId, { status: 'ready', progress: 100 });
    });

    const unlistenFailed = await listen<{ trackId: number; error: string }>('download:failed', (event) => {
      console.error('Download failed:', event.payload.trackId, event.payload.error);
      setDownloadState(event.payload.trackId, {
        status: 'failed',
        progress: 0,
        error: event.payload.error,
      });
    });

    unlisteners = [unlistenStarted, unlistenProgress, unlistenCompleted, unlistenFailed];
  } catch (err) {
    console.error('Failed to setup download event listeners:', err);
  }
}

// Stop listening for events
export function stopDownloadEventListeners(): void {
  for (const unlisten of unlisteners) {
    unlisten();
  }
  unlisteners = [];
}

// Start a download
export async function downloadTrack(track: {
  id: number;
  title: string;
  artist: string;
  album?: string;
  albumId?: string;
  durationSecs: number;
  quality: string;
  bitDepth?: number;
  sampleRate?: number;
}): Promise<void> {
  try {
    setDownloadState(track.id, { status: 'queued', progress: 0 });
    await invoke('download_track', {
      trackId: track.id,
      title: track.title,
      artist: track.artist,
      album: track.album,
      albumId: track.albumId,
      durationSecs: track.durationSecs,
      quality: track.quality,
      bitDepth: track.bitDepth,
      sampleRate: track.sampleRate,
    });
  } catch (err) {
    console.error('Failed to start download:', err);
    setDownloadState(track.id, { status: 'failed', progress: 0, error: String(err) });
    throw err;
  }
}

// Remove a download
export async function removeDownload(trackId: number): Promise<void> {
  try {
    await invoke('remove_downloaded_track', { trackId });
    downloadStates.delete(trackId);
    notifyListeners();
  } catch (err) {
    console.error('Failed to remove download:', err);
    throw err;
  }
}

// Get cache stats
export async function getDownloadCacheStats(): Promise<DownloadCacheStats> {
  return invoke<DownloadCacheStats>('get_download_cache_stats');
}

// Clear all downloads
export async function clearDownloadCache(): Promise<void> {
  await invoke('clear_download_cache');
  downloadStates.clear();
  notifyListeners();
}

// Open cache folder
export async function openDownloadCacheFolder(): Promise<void> {
  await invoke('open_download_cache_folder');
}

// Set cache limit
export async function setDownloadCacheLimit(limitMb: number | null): Promise<void> {
  await invoke('set_download_cache_limit', { limitMb });
}
