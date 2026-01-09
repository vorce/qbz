<script lang="ts">
  import { ChevronRight, MoreHorizontal } from 'lucide-svelte';

  interface Props {
    onPlayNow?: () => void;
    onPlayNext?: () => void;
    onPlayLater?: () => void;
    onAddFavorite?: () => void;
    onAddToPlaylist?: () => void;
    onShareQobuz?: () => void;
    onShareSonglink?: () => void;
    onGoToAlbum?: () => void;
    onGoToArtist?: () => void;
  }

  let {
    onPlayNow,
    onPlayNext,
    onPlayLater,
    onAddFavorite,
    onAddToPlaylist,
    onShareQobuz,
    onShareSonglink,
    onGoToAlbum,
    onGoToArtist
  }: Props = $props();

  let isOpen = $state(false);
  let shareOpen = $state(false);
  let menuRef: HTMLDivElement | null = null;

  const hasPlayback = $derived(!!(onPlayNow || onPlayNext || onPlayLater));
  const hasLibrary = $derived(!!(onAddFavorite || onAddToPlaylist));
  const hasShare = $derived(!!(onShareQobuz || onShareSonglink));
  const hasNav = $derived(!!(onGoToAlbum || onGoToArtist));
  const hasMenu = $derived(hasPlayback || hasLibrary || hasShare || hasNav);

  function closeMenu() {
    isOpen = false;
    shareOpen = false;
  }

  function handleClickOutside(event: MouseEvent) {
    if (menuRef && !menuRef.contains(event.target as Node)) {
      closeMenu();
    }
  }

  function handleAction(action?: () => void) {
    if (!action) return;
    action();
    closeMenu();
  }

  $effect(() => {
    if (isOpen) {
      document.addEventListener('mousedown', handleClickOutside);
      return () => document.removeEventListener('mousedown', handleClickOutside);
    }
  });
</script>

{#if hasMenu}
  <div
    class="track-menu"
    bind:this={menuRef}
    onmousedown={(e) => e.stopPropagation()}
    onclick={(e) => e.stopPropagation()}
  >
    <button
      class="menu-trigger"
      onclick={(e) => {
        e.stopPropagation();
        isOpen = !isOpen;
        shareOpen = false;
      }}
      aria-label="Track actions"
    >
      <MoreHorizontal size={18} />
    </button>

    {#if isOpen}
      <div class="menu">
        {#if hasPlayback}
          {#if onPlayNow}
            <button class="menu-item" onclick={() => handleAction(onPlayNow)}>Play now</button>
          {/if}
          {#if onPlayNext}
            <button class="menu-item" onclick={() => handleAction(onPlayNext)}>Play next</button>
          {/if}
          {#if onPlayLater}
            <button class="menu-item" onclick={() => handleAction(onPlayLater)}>Play later</button>
          {/if}
        {/if}

        {#if hasPlayback && (hasLibrary || hasShare || hasNav)}
          <div class="separator"></div>
        {/if}

        {#if hasLibrary}
          {#if onAddFavorite}
            <button class="menu-item" onclick={() => handleAction(onAddFavorite)}>Add to favorites</button>
          {/if}
          {#if onAddToPlaylist}
            <button class="menu-item" onclick={() => handleAction(onAddToPlaylist)}>Add to playlist</button>
          {/if}
        {/if}

        {#if hasLibrary && (hasShare || hasNav)}
          <div class="separator"></div>
        {/if}

        {#if hasShare}
          <div
            class="menu-item submenu-trigger"
            onmouseenter={() => (shareOpen = true)}
            onmouseleave={() => (shareOpen = false)}
            onclick={() => (shareOpen = !shareOpen)}
          >
            <span>Share</span>
            <ChevronRight size={14} />
            {#if shareOpen}
              <div class="submenu">
                {#if onShareQobuz}
                  <button class="menu-item" onclick={() => handleAction(onShareQobuz)}>Share Qobuz link</button>
                {/if}
                {#if onShareSonglink}
                  <button class="menu-item" onclick={() => handleAction(onShareSonglink)}>Share Song.link</button>
                {/if}
              </div>
            {/if}
          </div>
        {/if}

        {#if hasShare && hasNav}
          <div class="separator"></div>
        {/if}

        {#if hasNav}
          {#if onGoToAlbum}
            <button class="menu-item" onclick={() => handleAction(onGoToAlbum)}>Go to album</button>
          {/if}
          {#if onGoToArtist}
            <button class="menu-item" onclick={() => handleAction(onGoToArtist)}>Go to artist</button>
          {/if}
        {/if}
      </div>
    {/if}
  </div>
{/if}

<style>
  .track-menu {
    position: relative;
    display: inline-flex;
    align-items: center;
  }

  .menu-trigger {
    width: 32px;
    height: 32px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    border-radius: 6px;
    transition: background-color 150ms ease, color 150ms ease;
  }

  .menu-trigger:hover {
    background-color: var(--bg-hover);
    color: var(--text-primary);
  }

  .menu {
    position: absolute;
    right: 0;
    top: calc(100% + 6px);
    min-width: 200px;
    background-color: var(--bg-tertiary);
    border-radius: 10px;
    padding: 6px 0;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
    z-index: 60;
  }

  .menu-item {
    width: 100%;
    padding: 10px 16px;
    background: none;
    border: none;
    color: var(--text-secondary);
    text-align: left;
    font-size: 13px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    transition: background-color 150ms ease, color 150ms ease;
  }

  .menu-item:hover {
    background-color: var(--bg-hover);
    color: var(--text-primary);
  }

  .separator {
    height: 1px;
    background-color: var(--bg-hover);
    margin: 6px 0;
  }

  .submenu-trigger {
    position: relative;
  }

  .submenu {
    position: absolute;
    left: 100%;
    top: -6px;
    margin-left: 6px;
    min-width: 200px;
    background-color: var(--bg-tertiary);
    border-radius: 10px;
    padding: 6px 0;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
    z-index: 70;
  }
</style>
