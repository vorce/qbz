<script lang="ts">
  import { onMount } from 'svelte';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api/core';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { SkipBack, SkipForward, Play, Pause, X, Pin, PinOff } from 'lucide-svelte';

  interface TrackUpdate {
    id: number;
    title: string;
    artist: string;
    artwork: string;
    isPlaying: boolean;
  }

  interface PlaybackState {
    isPlaying: boolean;
    currentTime: number;
    duration: number;
  }

  // State
  let currentTrack = $state<TrackUpdate | null>(null);
  let isPlaying = $state(false);
  let currentTime = $state(0);
  let duration = $state(0);
  let isPinned = $state(true);
  let isDragging = $state(false);

  // Progress percentage
  const progress = $derived(duration > 0 ? (currentTime / duration) * 100 : 0);

  onMount(() => {
    const unlisteners: UnlistenFn[] = [];

    // Listen for track updates from main window
    listen<TrackUpdate>('miniplayer:track', (event) => {
      currentTrack = event.payload;
      isPlaying = event.payload.isPlaying;
    }).then((unlisten) => unlisteners.push(unlisten));

    // Listen for playback state updates
    listen<PlaybackState>('miniplayer:playback', (event) => {
      isPlaying = event.payload.isPlaying;
      currentTime = event.payload.currentTime;
      duration = event.payload.duration;
    }).then((unlisten) => unlisteners.push(unlisten));

    return () => {
      unlisteners.forEach((unlisten) => unlisten());
    };
  });

  async function handlePlayPause(): Promise<void> {
    try {
      if (isPlaying) {
        await invoke('pause_playback');
      } else {
        await invoke('resume_playback');
      }
    } catch (err) {
      console.error('[MiniPlayer] Failed to toggle playback:', err);
    }
  }

  async function handleNext(): Promise<void> {
    try {
      await invoke('next_track');
    } catch (err) {
      console.error('[MiniPlayer] Failed to skip to next:', err);
    }
  }

  async function handlePrevious(): Promise<void> {
    try {
      await invoke('previous_track');
    } catch (err) {
      console.error('[MiniPlayer] Failed to skip to previous:', err);
    }
  }

  async function handleClose(): Promise<void> {
    try {
      const window = getCurrentWindow();
      await window.hide();
    } catch (err) {
      console.error('[MiniPlayer] Failed to close:', err);
    }
  }

  async function togglePin(): Promise<void> {
    try {
      isPinned = !isPinned;
      const window = getCurrentWindow();
      await window.setAlwaysOnTop(isPinned);
    } catch (err) {
      console.error('[MiniPlayer] Failed to toggle pin:', err);
    }
  }

  async function startDrag(): Promise<void> {
    try {
      isDragging = true;
      const window = getCurrentWindow();
      await window.startDragging();
    } catch (err) {
      console.error('[MiniPlayer] Failed to start dragging:', err);
    } finally {
      isDragging = false;
    }
  }
</script>

<div
  class="miniplayer"
  class:dragging={isDragging}
  role="application"
  aria-label="MiniPlayer"
>
  <!-- Drag handle / Header -->
  <div
    class="header"
    onmousedown={startDrag}
    role="banner"
  >
    <button class="header-btn" onclick={togglePin} title={isPinned ? 'Unpin' : 'Pin on top'}>
      {#if isPinned}
        <Pin size={12} />
      {:else}
        <PinOff size={12} />
      {/if}
    </button>
    <button class="header-btn close" onclick={handleClose} title="Close">
      <X size={12} />
    </button>
  </div>

  <!-- Content -->
  <div class="content">
    <!-- Album Art -->
    <div class="artwork">
      {#if currentTrack?.artwork}
        <img src={currentTrack.artwork} alt="Album art" />
      {:else}
        <div class="artwork-placeholder"></div>
      {/if}
    </div>

    <!-- Track Info -->
    <div class="track-info">
      <div class="title">{currentTrack?.title ?? 'No track'}</div>
      <div class="artist">{currentTrack?.artist ?? '-'}</div>
    </div>

    <!-- Controls -->
    <div class="controls">
      <button class="control-btn" onclick={handlePrevious} title="Previous">
        <SkipBack size={16} />
      </button>
      <button class="control-btn play" onclick={handlePlayPause} title={isPlaying ? 'Pause' : 'Play'}>
        {#if isPlaying}
          <Pause size={18} />
        {:else}
          <Play size={18} />
        {/if}
      </button>
      <button class="control-btn" onclick={handleNext} title="Next">
        <SkipForward size={16} />
      </button>
    </div>
  </div>

  <!-- Progress Bar -->
  <div class="progress-container">
    <div class="progress-bar" style="width: {progress}%"></div>
  </div>
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    background: transparent;
    overflow: hidden;
  }

  .miniplayer {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    background: linear-gradient(135deg, rgba(26, 26, 30, 0.98) 0%, rgba(38, 38, 45, 0.98) 100%);
    border-radius: 12px;
    color: white;
    user-select: none;
    overflow: hidden;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
    border: 1px solid rgba(255, 255, 255, 0.1);
  }

  .miniplayer.dragging {
    cursor: grabbing;
  }

  .header {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: 4px;
    padding: 4px 6px;
    cursor: grab;
    background: rgba(0, 0, 0, 0.2);
  }

  .header:active {
    cursor: grabbing;
  }

  .header-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 18px;
    height: 18px;
    background: transparent;
    border: none;
    color: rgba(255, 255, 255, 0.5);
    cursor: pointer;
    border-radius: 4px;
    transition: all 0.15s ease;
  }

  .header-btn:hover {
    background: rgba(255, 255, 255, 0.1);
    color: rgba(255, 255, 255, 0.8);
  }

  .header-btn.close:hover {
    background: rgba(255, 100, 100, 0.3);
    color: #ff6b6b;
  }

  .content {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 6px 12px 8px;
    flex: 1;
  }

  .artwork {
    width: 48px;
    height: 48px;
    border-radius: 6px;
    overflow: hidden;
    flex-shrink: 0;
    background: rgba(255, 255, 255, 0.05);
  }

  .artwork img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .artwork-placeholder {
    width: 100%;
    height: 100%;
    background: linear-gradient(135deg, #333 0%, #222 100%);
  }

  .track-info {
    flex: 1;
    min-width: 0;
    overflow: hidden;
  }

  .title {
    font-weight: 500;
    font-size: 13px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    color: #fff;
  }

  .artist {
    font-size: 11px;
    color: rgba(255, 255, 255, 0.6);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    margin-top: 2px;
  }

  .controls {
    display: flex;
    align-items: center;
    gap: 4px;
    flex-shrink: 0;
  }

  .control-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    background: transparent;
    border: none;
    color: rgba(255, 255, 255, 0.8);
    cursor: pointer;
    border-radius: 50%;
    transition: all 0.15s ease;
  }

  .control-btn:hover {
    background: rgba(255, 255, 255, 0.1);
    color: #fff;
  }

  .control-btn:active {
    transform: scale(0.95);
  }

  .control-btn.play {
    width: 32px;
    height: 32px;
    background: rgba(255, 255, 255, 0.15);
  }

  .control-btn.play:hover {
    background: rgba(255, 255, 255, 0.25);
  }

  .progress-container {
    height: 3px;
    background: rgba(255, 255, 255, 0.1);
  }

  .progress-bar {
    height: 100%;
    background: linear-gradient(90deg, #6366f1 0%, #8b5cf6 100%);
    transition: width 0.25s linear;
  }
</style>
