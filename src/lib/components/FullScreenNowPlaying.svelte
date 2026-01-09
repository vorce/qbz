<script lang="ts">
  import { X, Shuffle, SkipBack, Play, Pause, SkipForward, Repeat, Heart, List, Mic2, Maximize2, MoreHorizontal } from 'lucide-svelte';

  interface Props {
    isOpen: boolean;
    onClose: () => void;
    artwork: string;
    trackTitle: string;
    artist: string;
    album: string;
    quality: string;
    qualityLevel: number;
    isPlaying: boolean;
    onTogglePlay: () => void;
    currentTime: number;
    duration: number;
    onSeek: (time: number) => void;
    volume: number;
    onVolumeChange: (volume: number) => void;
    isShuffle: boolean;
    onToggleShuffle: () => void;
    repeatMode: 'off' | 'all' | 'one';
    onToggleRepeat: () => void;
    isFavorite: boolean;
    onToggleFavorite: () => void;
    onOpenQueue?: () => void;
    onOpenFocusMode?: () => void;
  }

  let {
    isOpen,
    onClose,
    artwork,
    trackTitle,
    artist,
    album,
    quality,
    qualityLevel,
    isPlaying,
    onTogglePlay,
    currentTime,
    duration,
    onSeek,
    volume,
    onVolumeChange,
    isShuffle,
    onToggleShuffle,
    repeatMode,
    onToggleRepeat,
    isFavorite,
    onToggleFavorite,
    onOpenQueue,
    onOpenFocusMode
  }: Props = $props();

  let progressRef: HTMLDivElement;
  let volumeRef: HTMLDivElement;
  let isDraggingProgress = $state(false);
  let isDraggingVolume = $state(false);
  let playBtnHovered = $state(false);

  const progress = $derived((currentTime / duration) * 100);

  function formatTime(seconds: number): string {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  }

  function handleProgressMouseDown(e: MouseEvent) {
    isDraggingProgress = true;
    updateProgress(e);
  }

  function updateProgress(e: MouseEvent) {
    if (progressRef) {
      const rect = progressRef.getBoundingClientRect();
      const percentage = Math.max(0, Math.min(100, ((e.clientX - rect.left) / rect.width) * 100));
      onSeek(Math.round((percentage / 100) * duration));
    }
  }

  function handleVolumeMouseDown(e: MouseEvent) {
    isDraggingVolume = true;
    updateVolume(e);
  }

  function updateVolume(e: MouseEvent) {
    if (volumeRef) {
      const rect = volumeRef.getBoundingClientRect();
      const percentage = Math.max(0, Math.min(100, ((e.clientX - rect.left) / rect.width) * 100));
      onVolumeChange(Math.round(percentage));
    }
  }

  function handleMouseMove(e: MouseEvent) {
    if (isDraggingProgress) updateProgress(e);
    if (isDraggingVolume) updateVolume(e);
  }

  function handleMouseUp() {
    isDraggingProgress = false;
    isDraggingVolume = false;
  }

  $effect(() => {
    if (isDraggingProgress || isDraggingVolume) {
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
      return () => {
        document.removeEventListener('mousemove', handleMouseMove);
        document.removeEventListener('mouseup', handleMouseUp);
      };
    }
  });
</script>

{#if isOpen}
  <div class="fullscreen-player">
    <!-- Close Button -->
    <button class="close-btn" onclick={onClose}>
      <X size={24} />
    </button>

    <!-- Album Artwork -->
    <div class="artwork">
      <img src={artwork} alt={trackTitle} />
    </div>

    <!-- Track Info -->
    <div class="track-info">
      <h1 class="title">{trackTitle}</h1>
      <h2 class="artist">{artist}</h2>
      <h3 class="album">{album}</h3>
    </div>

    <!-- Quality Info -->
    <div class="quality-info">
      <div class="quality-text">{quality}</div>
      <div class="quality-dots">
        {#each Array(5) as _, i}
          <div class="dot" class:active={i < qualityLevel}></div>
        {/each}
      </div>
    </div>

    <!-- Progress Bar -->
    <div class="progress-container">
      <div
        class="progress-bar"
        bind:this={progressRef}
        onmousedown={handleProgressMouseDown}
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

    <!-- Playback Controls -->
    <div class="controls">
      <button class="control-btn" class:active={isShuffle} onclick={onToggleShuffle}>
        <Shuffle size={24} />
      </button>
      <button class="control-btn primary">
        <SkipBack size={32} />
      </button>
      <button
        class="play-btn"
        style="background-color: {playBtnHovered ? 'var(--accent-hover)' : 'var(--accent-primary)'}"
        onmouseenter={() => (playBtnHovered = true)}
        onmouseleave={() => (playBtnHovered = false)}
        onclick={onTogglePlay}
      >
        {#if isPlaying}
          <Pause size={28} fill="white" color="white" />
        {:else}
          <Play size={28} fill="white" color="white" class="play-icon" />
        {/if}
      </button>
      <button class="control-btn primary">
        <SkipForward size={32} />
      </button>
      <button class="control-btn" class:active={repeatMode !== 'off'} onclick={onToggleRepeat}>
        <Repeat size={24} />
        {#if repeatMode === 'one'}
          <span class="repeat-one">1</span>
        {/if}
      </button>
    </div>

    <!-- Volume Control -->
    <div class="volume-container">
      <div
        class="volume-bar"
        bind:this={volumeRef}
        onmousedown={handleVolumeMouseDown}
        role="slider"
        tabindex="0"
        aria-valuenow={volume}
        aria-valuemin={0}
        aria-valuemax={100}
      >
        <div class="volume-fill" style="width: {volume}%"></div>
        <div class="volume-thumb" style="left: {volume}%"></div>
      </div>
    </div>

    <!-- Bottom Actions -->
    <div class="bottom-actions">
      <button class="action-btn" onclick={onOpenQueue}>
        <List size={24} />
      </button>
      <button class="action-btn" onclick={onOpenFocusMode}>
        <Maximize2 size={24} />
      </button>
      <button class="action-btn">
        <Mic2 size={24} />
      </button>
      <button class="action-btn" onclick={onToggleFavorite}>
        <Heart size={24} fill={isFavorite ? 'var(--accent-primary)' : 'none'} color={isFavorite ? 'var(--accent-primary)' : '#888888'} />
      </button>
      <button class="action-btn">
        <MoreHorizontal size={24} />
      </button>
    </div>
  </div>
{/if}

<style>
  .fullscreen-player {
    position: fixed;
    inset: 0;
    z-index: 100;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background-color: #0a0a0a;
    animation: fadeIn 200ms ease-out;
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
    color: var(--text-muted);
    cursor: pointer;
    transition: all 150ms ease;
  }

  .close-btn:hover {
    color: var(--text-primary);
    background-color: rgba(255, 255, 255, 0.1);
  }

  .artwork {
    width: 400px;
    height: 400px;
    border-radius: 16px;
    overflow: hidden;
    box-shadow: 0 16px 48px rgba(0, 0, 0, 0.6);
    margin-bottom: 32px;
  }

  .artwork img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .track-info {
    text-align: center;
    margin-bottom: 8px;
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
    margin-bottom: 4px;
  }

  .album {
    font-size: 16px;
    color: #666666;
  }

  .quality-info {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    margin-bottom: 32px;
  }

  .quality-text {
    font-size: 14px;
    color: #666666;
  }

  .quality-dots {
    display: flex;
    align-items: center;
    gap: 3px;
  }

  .dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background-color: #444444;
  }

  .dot.active {
    background-color: #888888;
  }

  .progress-container {
    width: 100%;
    max-width: 600px;
    margin-bottom: 24px;
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

  .controls {
    display: flex;
    align-items: center;
    gap: 24px;
    margin-bottom: 32px;
  }

  .control-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    transition: color 150ms ease;
    position: relative;
  }

  .control-btn:hover {
    color: var(--text-primary);
  }

  .control-btn.primary {
    color: var(--text-primary);
  }

  .control-btn.primary:hover {
    color: var(--text-muted);
  }

  .control-btn.active {
    color: var(--accent-primary);
  }

  .repeat-one {
    position: absolute;
    top: -4px;
    right: -4px;
    font-size: 10px;
    font-weight: 700;
  }

  .play-btn {
    width: 64px;
    height: 64px;
    border-radius: 50%;
    border: none;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: background-color 150ms ease;
  }

  .play-btn :global(.play-icon) {
    margin-left: 2px;
  }

  .volume-container {
    width: 100%;
    max-width: 200px;
    margin-bottom: 32px;
  }

  .volume-bar {
    height: 4px;
    background-color: #333333;
    border-radius: 9999px;
    position: relative;
    cursor: pointer;
  }

  .volume-fill {
    height: 100%;
    background-color: var(--accent-primary);
    border-radius: 9999px;
  }

  .volume-thumb {
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

  .volume-bar:hover .volume-thumb {
    opacity: 1;
  }

  .bottom-actions {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .action-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    transition: color 150ms ease;
  }

  .action-btn:hover {
    color: var(--text-primary);
  }
</style>
