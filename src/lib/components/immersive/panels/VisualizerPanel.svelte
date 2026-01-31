<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api/core';
  import { Settings } from 'lucide-svelte';

  type VisualizerMode = 'bars' | 'wave' | 'mirror';

  interface Props {
    isPlaying?: boolean;
    artwork?: string;
  }

  let { isPlaying = false, artwork }: Props = $props();

  // FFT bins from Rust backend (32 values, 0.0 to 1.0)
  let bins: number[] = $state(Array(32).fill(0));
  let mode: VisualizerMode = $state('bars');
  let isEnabled = $state(false);

  // Colors extracted from artwork or defaults
  let primaryColor = $state('#7c3aed');
  let secondaryColor = $state('#a78bfa');

  // Extract dominant color from artwork
  async function extractColor(imageUrl: string) {
    try {
      const img = new Image();
      img.crossOrigin = 'anonymous';

      await new Promise<void>((resolve, reject) => {
        img.onload = () => resolve();
        img.onerror = reject;
        img.src = imageUrl;
      });

      const canvas = document.createElement('canvas');
      canvas.width = 10;
      canvas.height = 10;
      const ctx = canvas.getContext('2d');
      if (!ctx) return;

      ctx.drawImage(img, 0, 0, 10, 10);
      const data = ctx.getImageData(0, 0, 10, 10).data;

      // Get color from center
      const idx = (5 * 10 + 5) * 4;
      const r = Math.min(255, Math.round(data[idx] * 1.2));
      const g = Math.min(255, Math.round(data[idx + 1] * 1.2));
      const b = Math.min(255, Math.round(data[idx + 2] * 1.2));

      primaryColor = `rgb(${r}, ${g}, ${b})`;
      // Lighter version for secondary
      secondaryColor = `rgb(${Math.min(255, r + 60)}, ${Math.min(255, g + 60)}, ${Math.min(255, b + 60)})`;
    } catch {
      primaryColor = '#7c3aed';
      secondaryColor = '#a78bfa';
    }
  }

  // Watch artwork changes
  $effect(() => {
    if (artwork) {
      extractColor(artwork);
    }
  });

  // Enable visualizer when component mounts and playing
  $effect(() => {
    if (isPlaying && !isEnabled) {
      invoke('set_visualizer_enabled', { enabled: true });
      isEnabled = true;
    } else if (!isPlaying && isEnabled) {
      // Keep enabled but bins will naturally decay
    }
  });

  function cycleMode() {
    const modes: VisualizerMode[] = ['bars', 'wave', 'mirror'];
    const currentIndex = modes.indexOf(mode);
    mode = modes[(currentIndex + 1) % modes.length];
  }

  onMount(() => {
    // Enable visualizer on mount
    invoke('set_visualizer_enabled', { enabled: true });
    isEnabled = true;

    // Listen for spectrum data from Rust
    const unlisten = listen<number[]>('audio-spectrum', (event) => {
      bins = event.payload;
    });

    return () => {
      // Disable visualizer on unmount
      invoke('set_visualizer_enabled', { enabled: false });
      isEnabled = false;
      unlisten.then(fn => fn());
    };
  });
</script>

<div class="visualizer-panel">
  {#if mode === 'bars'}
    <div class="bars-container">
      {#each bins as bin, i}
        <div
          class="bar"
          style="
            --height: {Math.max(5, bin * 100)}%;
            --color: {primaryColor};
            --delay: {i * 10}ms;
          "
        ></div>
      {/each}
    </div>
  {:else if mode === 'wave'}
    <div class="wave-container">
      <svg viewBox="0 0 320 100" preserveAspectRatio="none" class="wave-svg">
        <defs>
          <linearGradient id="waveGradient" x1="0%" y1="100%" x2="0%" y2="0%">
            <stop offset="0%" style="stop-color:{primaryColor};stop-opacity:0.8" />
            <stop offset="100%" style="stop-color:{secondaryColor};stop-opacity:0.4" />
          </linearGradient>
        </defs>
        <path
          d={`M 0 50 ${bins.map((bin, i) => {
            const x = (i / (bins.length - 1)) * 320;
            const y = 50 - bin * 45;
            return `L ${x} ${y}`;
          }).join(' ')} L 320 50`}
          fill="none"
          stroke="url(#waveGradient)"
          stroke-width="3"
          stroke-linecap="round"
          stroke-linejoin="round"
        />
      </svg>
    </div>
  {:else if mode === 'mirror'}
    <div class="mirror-container">
      {#each bins as bin, i}
        <div class="mirror-bar-wrapper">
          <div
            class="mirror-bar top"
            style="--height: {Math.max(3, bin * 100)}%; --color: {primaryColor};"
          ></div>
          <div
            class="mirror-bar bottom"
            style="--height: {Math.max(3, bin * 100)}%; --color: {secondaryColor};"
          ></div>
        </div>
      {/each}
    </div>
  {/if}

  <!-- Mode indicator -->
  <button class="mode-btn" onclick={cycleMode} title="Change visualizer style">
    <Settings size={18} />
    <span class="mode-label">{mode}</span>
  </button>
</div>

<style>
  .visualizer-panel {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 80px 40px 140px;
    z-index: 5;
  }

  /* Bars mode - GPU accelerated with transform */
  .bars-container {
    display: flex;
    align-items: flex-end;
    justify-content: center;
    gap: 4px;
    height: 60%;
    width: 90%;
    max-width: 800px;
  }

  .bar {
    flex: 1;
    max-width: 20px;
    height: var(--height);
    background: linear-gradient(to top, var(--color), transparent);
    border-radius: 4px 4px 0 0;
    /* GPU accelerated - only transform, no layout */
    transform: scaleY(1);
    transform-origin: bottom;
    transition: height 80ms ease-out;
    will-change: height;
  }

  /* Wave mode */
  .wave-container {
    width: 90%;
    max-width: 900px;
    height: 40%;
  }

  .wave-svg {
    width: 100%;
    height: 100%;
  }

  .wave-svg path {
    transition: d 80ms ease-out;
  }

  /* Mirror mode - bars going up and down from center */
  .mirror-container {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 3px;
    height: 70%;
    width: 90%;
    max-width: 800px;
  }

  .mirror-bar-wrapper {
    flex: 1;
    max-width: 18px;
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 2px;
  }

  .mirror-bar {
    width: 100%;
    height: var(--height);
    max-height: 48%;
    background: linear-gradient(to top, var(--color), transparent);
    border-radius: 3px;
    transition: height 80ms ease-out;
    will-change: height;
  }

  .mirror-bar.top {
    transform-origin: bottom;
  }

  .mirror-bar.bottom {
    transform: scaleY(-1);
    transform-origin: top;
  }

  /* Mode button */
  .mode-btn {
    position: absolute;
    bottom: 160px;
    right: 40px;
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 14px;
    background: rgba(0, 0, 0, 0.4);
    backdrop-filter: blur(12px);
    border: 1px solid var(--alpha-15, rgba(255, 255, 255, 0.15));
    border-radius: 20px;
    color: var(--alpha-60, rgba(255, 255, 255, 0.6));
    font-size: 12px;
    cursor: pointer;
    transition: all 150ms ease;
    text-transform: capitalize;
  }

  .mode-btn:hover {
    background: rgba(0, 0, 0, 0.5);
    color: var(--text-primary, white);
  }

  .mode-label {
    font-weight: 500;
  }

  /* Responsive */
  @media (max-width: 768px) {
    .visualizer-panel {
      padding: 70px 20px 130px;
    }

    .bars-container {
      gap: 2px;
    }

    .bar {
      max-width: 12px;
    }

    .mirror-bar-wrapper {
      max-width: 10px;
    }

    .mode-btn {
      bottom: 145px;
      right: 20px;
    }
  }
</style>
