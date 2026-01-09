<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { X, SkipBack, Play, Pause, SkipForward } from 'lucide-svelte';

  interface Props {
    isOpen: boolean;
    onClose: () => void;
    artwork: string;
    trackTitle: string;
    artist: string;
    isPlaying: boolean;
    onTogglePlay: () => void;
    currentTime: number;
    duration: number;
    onSeek: (time: number) => void;
  }

  let {
    isOpen,
    onClose,
    artwork,
    trackTitle,
    artist,
    isPlaying,
    onTogglePlay,
    currentTime,
    duration,
    onSeek
  }: Props = $props();

  let showControls = $state(false);
  let hideTimeout: ReturnType<typeof setTimeout> | null = null;
  let progressRef: HTMLDivElement;

  const progress = $derived((currentTime / duration) * 100);

  function formatTime(seconds: number): string {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  }

  function handleProgressClick(e: MouseEvent) {
    if (progressRef) {
      const rect = progressRef.getBoundingClientRect();
      const percentage = Math.max(0, Math.min(100, ((e.clientX - rect.left) / rect.width) * 100));
      onSeek(Math.round((percentage / 100) * duration));
    }
  }

  function showControlsTemporarily() {
    showControls = true;
    if (hideTimeout) clearTimeout(hideTimeout);
    hideTimeout = setTimeout(() => (showControls = false), 3000);
  }

  function handleKeyPress(e: KeyboardEvent) {
    if (!isOpen) return;
    if (e.key === 'Escape') {
      onClose();
    } else if (e.key === ' ') {
      e.preventDefault();
      onTogglePlay();
    }
  }

  function handleMouseMove() {
    showControlsTemporarily();
  }

  $effect(() => {
    if (isOpen) {
      document.addEventListener('keydown', handleKeyPress);
      document.addEventListener('mousemove', handleMouseMove);
      return () => {
        document.removeEventListener('keydown', handleKeyPress);
        document.removeEventListener('mousemove', handleMouseMove);
        if (hideTimeout) clearTimeout(hideTimeout);
      };
    }
  });
</script>

{#if isOpen}
  <div class="focus-mode" onclick={showControlsTemporarily} role="presentation">
    <!-- Close Button -->
    <button class="close-btn" class:visible={showControls} onclick={onClose}>
      <X size={32} />
    </button>

    <!-- Album Artwork -->
    <div class="artwork-container">
      <img src={artwork} alt={trackTitle} />

      <!-- Overlay Controls -->
      <div class="overlay" class:visible={showControls}>
        <div class="playback-controls">
          <button class="nav-btn">
            <SkipBack size={40} />
          </button>
          <button
            class="play-btn"
            onclick={(e) => {
              e.stopPropagation();
              onTogglePlay();
            }}
          >
            {#if isPlaying}
              <Pause size={36} fill="white" color="white" />
            {:else}
              <Play size={36} fill="white" color="white" class="play-icon" />
            {/if}
          </button>
          <button class="nav-btn">
            <SkipForward size={40} />
          </button>
        </div>
      </div>
    </div>

    <!-- Track Info -->
    <div class="track-info">
      <h1 class="title">{trackTitle}</h1>
      <h2 class="artist">{artist}</h2>
    </div>

    <!-- Progress Bar -->
    <div class="progress-container" class:visible={showControls}>
      <div
        class="progress-bar"
        bind:this={progressRef}
        onclick={handleProgressClick}
        role="slider"
        tabindex="0"
        aria-valuenow={currentTime}
        aria-valuemin={0}
        aria-valuemax={duration}
      >
        <div class="progress-fill" style="width: {progress}%"></div>
        <div class="progress-thumb" style="left: {progress}%"></div>
      </div>
      <div class="time-display">
        <span>{formatTime(currentTime)}</span>
        <span>{formatTime(duration)}</span>
      </div>
    </div>
  </div>
{/if}

<style>
  .focus-mode {
    position: fixed;
    inset: 0;
    z-index: 110;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background-color: #0a0a0a;
    cursor: default;
    animation: fadeIn 300ms ease-out;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  .close-btn {
    position: absolute;
    top: 24px;
    right: 24px;
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: none;
    border: none;
    border-radius: 50%;
    color: #666666;
    cursor: pointer;
    opacity: 0;
    transition: all 300ms ease;
  }

  .close-btn.visible {
    opacity: 1;
  }

  .close-btn:hover {
    color: var(--text-primary);
    background-color: rgba(255, 255, 255, 0.1);
  }

  .artwork-container {
    position: relative;
    width: 500px;
    height: 500px;
    border-radius: 16px;
    overflow: hidden;
    box-shadow: 0 16px 48px rgba(0, 0, 0, 0.6);
    margin-bottom: 32px;
    animation: scaleIn 400ms ease-out;
  }

  @keyframes scaleIn {
    from {
      transform: scale(0.95);
      opacity: 0;
    }
    to {
      transform: scale(1);
      opacity: 1;
    }
  }

  .artwork-container img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .overlay {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: rgba(0, 0, 0, 0.4);
    opacity: 0;
    transition: opacity 300ms ease;
  }

  .overlay.visible {
    opacity: 1;
  }

  .playback-controls {
    display: flex;
    align-items: center;
    gap: 24px;
  }

  .nav-btn {
    background: none;
    border: none;
    color: rgba(255, 255, 255, 0.9);
    cursor: pointer;
    transition: color 150ms ease;
  }

  .nav-btn:hover {
    color: white;
  }

  .play-btn {
    width: 80px;
    height: 80px;
    border-radius: 50%;
    border: none;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: rgba(255, 255, 255, 0.2);
    backdrop-filter: blur(8px);
    cursor: pointer;
    transition: background-color 150ms ease;
  }

  .play-btn:hover {
    background-color: rgba(255, 255, 255, 0.3);
  }

  .play-btn :global(.play-icon) {
    margin-left: 3px;
  }

  .track-info {
    text-align: center;
    margin-bottom: 24px;
  }

  .title {
    font-size: 28px;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 8px;
  }

  .artist {
    font-size: 20px;
    color: var(--text-muted);
  }

  .progress-container {
    width: 100%;
    max-width: 600px;
    opacity: 0;
    transition: opacity 300ms ease;
  }

  .progress-container.visible {
    opacity: 1;
  }

  .progress-bar {
    height: 4px;
    background-color: #333333;
    border-radius: 9999px;
    position: relative;
    cursor: pointer;
    margin-bottom: 12px;
  }

  .progress-fill {
    height: 100%;
    background-color: var(--accent-primary);
    border-radius: 9999px;
  }

  .progress-thumb {
    position: absolute;
    top: 50%;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background-color: white;
    transform: translate(-50%, -50%);
    opacity: 0;
    transition: opacity 150ms ease;
  }

  .progress-bar:hover .progress-thumb {
    opacity: 1;
  }

  .time-display {
    display: flex;
    justify-content: space-between;
    font-size: 14px;
    color: #666666;
    font-family: var(--font-mono);
    font-variant-numeric: tabular-nums;
  }
</style>
