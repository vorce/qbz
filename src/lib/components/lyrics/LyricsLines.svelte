<script lang="ts">
  import { tick } from 'svelte';

  interface LyricsLine {
    text: string;
  }

  interface Props {
    lines: LyricsLine[];
    activeIndex?: number;
    activeProgress?: number;
    dimInactive?: boolean;
    center?: boolean;
    compact?: boolean;
    scrollToActive?: boolean;
  }

  let {
    lines,
    activeIndex = -1,
    activeProgress = 0,
    dimInactive = true,
    center = false,
    compact = false,
    scrollToActive = true
  }: Props = $props();

  let container: HTMLDivElement | null = null;
  let lastScrolledIndex = -1;

  // Calculate opacity based on distance from active line
  function getLineOpacity(index: number, active: number): number {
    if (!dimInactive || active < 0) return 1;
    if (index === active) return 1;

    const distance = Math.abs(index - active);
    // Fade out based on distance: 1 line away = 0.6, 2 = 0.4, 3+ = 0.25
    if (distance === 1) return 0.6;
    if (distance === 2) return 0.4;
    if (distance === 3) return 0.3;
    return 0.2;
  }

  // Calculate blur based on distance (subtle effect)
  function getLineBlur(index: number, active: number): number {
    if (!dimInactive || active < 0) return 0;
    if (index === active) return 0;

    const distance = Math.abs(index - active);
    if (distance <= 2) return 0;
    if (distance <= 4) return 0.5;
    return 1;
  }

  // Scroll active line into view
  async function scrollActiveIntoView() {
    if (!container || activeIndex < 0 || activeIndex === lastScrolledIndex) return;

    await tick(); // Wait for DOM update

    const target = container.querySelector<HTMLElement>(`[data-line-index="${activeIndex}"]`);
    if (!target) return;

    lastScrolledIndex = activeIndex;

    // Calculate scroll position to center the active line
    const containerRect = container.getBoundingClientRect();
    const targetRect = target.getBoundingClientRect();
    const targetCenter = targetRect.top + targetRect.height / 2;
    const containerCenter = containerRect.top + containerRect.height / 2;
    const scrollOffset = targetCenter - containerCenter;

    container.scrollBy({
      top: scrollOffset,
      behavior: 'smooth'
    });
  }

  // React to activeIndex changes
  $effect(() => {
    if (scrollToActive && activeIndex >= 0) {
      scrollActiveIntoView();
    }
  });
</script>

<div
  class="lyrics-lines"
  class:compact
  class:center
  bind:this={container}
>
  {#if lines.length === 0}
    <div class="lyrics-empty">No lyrics available</div>
  {:else}
    <!-- Spacer at top to allow first lines to scroll to center -->
    <div class="lyrics-spacer"></div>

    {#each lines as line, index}
      {@const isActive = index === activeIndex}
      {@const isPast = index < activeIndex}
      {@const opacity = getLineOpacity(index, activeIndex)}
      {@const blur = getLineBlur(index, activeIndex)}
      <div
        class="lyrics-line"
        class:active={isActive}
        class:past={isPast}
        style="
          --line-opacity: {opacity};
          --line-blur: {blur}px;
          {isActive ? `--line-progress: ${Math.max(0, Math.min(1, activeProgress))}` : ''}
        "
        data-line-index={index}
      >
        <span class="line-text">{line.text}</span>
      </div>
    {/each}

    <!-- Spacer at bottom to allow last lines to scroll to center -->
    <div class="lyrics-spacer"></div>
  {/if}
</div>

<style>
  .lyrics-lines {
    display: flex;
    flex-direction: column;
    gap: var(--lyrics-line-gap, 20px);
    padding: 16px 20px;
    overflow-y: auto;
    height: 100%;
    scroll-behavior: smooth;
    scrollbar-width: thin;
    scrollbar-color: var(--bg-tertiary) transparent;
  }

  .lyrics-lines::-webkit-scrollbar {
    width: 6px;
  }

  .lyrics-lines::-webkit-scrollbar-track {
    background: transparent;
  }

  .lyrics-lines::-webkit-scrollbar-thumb {
    background: var(--bg-tertiary);
    border-radius: 3px;
  }

  .lyrics-spacer {
    min-height: 40vh;
    flex-shrink: 0;
  }

  .lyrics-lines.center {
    text-align: center;
  }

  .lyrics-lines.compact {
    gap: 14px;
  }

  .lyrics-line {
    color: var(--text-secondary);
    font-family: var(--font-sans);
    font-size: 16px;
    line-height: 1.5;
    letter-spacing: 0.01em;
    opacity: var(--line-opacity, 1);
    filter: blur(var(--line-blur, 0));
    transition:
      opacity 300ms ease-out,
      filter 300ms ease-out,
      transform 300ms ease-out,
      font-size 200ms ease-out,
      color 200ms ease-out;
    transform-origin: left center;
  }

  .lyrics-lines.center .lyrics-line {
    transform-origin: center center;
  }

  .lyrics-line.past {
    color: var(--text-muted);
  }

  .lyrics-line.active {
    color: var(--text-primary);
    font-size: 22px;
    font-weight: 600;
    opacity: 1;
    filter: blur(0);
    transform: scale(1.02);
  }

  /* Karaoke progress effect on active line */
  .lyrics-line.active .line-text {
    background: linear-gradient(
      90deg,
      var(--accent-primary) calc(var(--line-progress, 0) * 100%),
      var(--text-primary) calc(var(--line-progress, 0) * 100%)
    );
    -webkit-background-clip: text;
    background-clip: text;
    color: transparent;
  }

  .lyrics-empty {
    color: var(--text-muted);
    font-size: 14px;
    text-align: center;
    padding: 48px 0;
  }
</style>
