<script lang="ts">
  import { onMount, tick } from 'svelte';
  import { Play, Disc3 } from 'lucide-svelte';

  interface Props {
    artwork: string;
    title: string;
    artist: string;
    quality?: string;
    size?: 'standard' | 'large';
    onclick?: () => void;
  }

  let { artwork, title, artist, quality, size = 'standard', onclick }: Props = $props();

  let imageError = $state(false);
  const cardSize = $derived(size === 'large' ? 180 : 162);
  let titleRef: HTMLDivElement | null = $state(null);
  let titleTextRef: HTMLSpanElement | null = $state(null);
  let titleOverflow = $state(0);
  const titleOffset = $derived(titleOverflow > 0 ? `-${titleOverflow + 16}px` : '0px');

  function handleImageError() {
    imageError = true;
  }

  function updateTitleOverflow() {
    if (!titleRef || !titleTextRef) return;
    const overflow = titleTextRef.scrollWidth - titleRef.clientWidth;
    titleOverflow = overflow > 0 ? overflow : 0;
  }

  onMount(() => {
    updateTitleOverflow();
    const observer = new ResizeObserver(() => updateTitleOverflow());
    if (titleRef) {
      observer.observe(titleRef);
    }
    return () => observer.disconnect();
  });

  $effect(() => {
    title;
    tick().then(updateTitleOverflow);
  });
</script>

<div
  class="album-card"
  style="width: {cardSize}px"
  onclick={onclick}
  role="button"
  tabindex="0"
  onkeydown={(e) => e.key === 'Enter' && onclick?.()}
>
  <!-- Artwork Container -->
  <div
    class="artwork-container"
    style="width: {cardSize}px; height: {cardSize}px"
  >
    {#if imageError || !artwork}
      <div class="artwork-placeholder">
        <Disc3 size={48} />
      </div>
    {:else}
      <img src={artwork} alt={title} loading="lazy" decoding="async" onerror={handleImageError} />
    {/if}

    <!-- Play Button Overlay (CSS-only hover) -->
    <div class="play-overlay">
      <div class="play-button">
        <Play size={24} fill="white" color="white" />
      </div>
    </div>
  </div>

  <!-- Text Info -->
  <div class="info">
    <div
      class="title"
      class:scrollable={titleOverflow > 0}
      style="--ticker-offset: {titleOffset};"
      bind:this={titleRef}
    >
      <span class="title-text" bind:this={titleTextRef}>{title}</span>
    </div>
    <div class="artist">{artist}</div>
    {#if quality}
      <div class="quality-badge">{quality}</div>
    {/if}
  </div>
</div>

<style>
  .album-card {
    flex-shrink: 0;
    cursor: pointer;
    transition: transform 150ms ease;
  }

  .artwork-container {
    position: relative;
    margin-bottom: 8px;
    border-radius: 8px;
    overflow: hidden;
    transition: transform 150ms ease-out;
  }

  .album-card:hover .artwork-container {
    transform: scale(1.02);
  }

  .artwork-container img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .artwork-placeholder {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, var(--bg-tertiary) 0%, var(--bg-secondary) 100%);
    color: var(--text-muted);
  }

  .quality-badge {
    display: inline-block;
    margin-top: 4px;
    font-size: 10px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.85);
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.15);
    border-radius: 4px;
    padding: 2px 6px;
  }

  .play-overlay {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0;
    transition: opacity 150ms ease;
  }

  .album-card:hover .play-overlay {
    opacity: 1;
  }

  .play-button {
    width: 48px;
    height: 48px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: rgba(66, 133, 244, 0.9);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
  }

  .info {
    width: 100%;
  }

  .title {
    font-size: 14px;
    font-weight: 500;
    color: var(--text-primary);
    line-height: 1.4;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    margin-bottom: 2px;
  }

  .title.scrollable {
    text-overflow: clip;
  }

  .title-text {
    display: inline-block;
    white-space: nowrap;
  }

  .album-card:hover .title.scrollable .title-text {
    animation: title-ticker 6s linear infinite;
    will-change: transform;
  }

  @keyframes title-ticker {
    from { transform: translateX(0); }
    to { transform: translateX(var(--ticker-offset)); }
  }

  .artist {
    font-size: 13px;
    font-weight: 400;
    color: var(--text-muted);
    line-height: 1.4;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
