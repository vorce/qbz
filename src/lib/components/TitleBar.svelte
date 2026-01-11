<script lang="ts">
  import { Minus, Square, X, Copy } from 'lucide-svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  let isMaximized = $state(false);

  const appWindow = getCurrentWindow();

  // Check initial maximized state
  appWindow.isMaximized().then((maximized) => {
    isMaximized = maximized;
  });

  // Listen for window state changes
  appWindow.onResized(async () => {
    isMaximized = await appWindow.isMaximized();
  });

  function handleMinimize() {
    appWindow.minimize();
  }

  function handleMaximize() {
    appWindow.toggleMaximize();
  }

  function handleClose() {
    appWindow.close();
  }
</script>

<div class="titlebar" data-tauri-drag-region>
  <div class="titlebar-content" data-tauri-drag-region>
    <img src="/icons/AppIcons/android/48x48.png" alt="QBZ" class="titlebar-icon" />
    <span class="titlebar-title">QBZ</span>
  </div>

  <div class="window-controls">
    <button class="control-btn" onclick={handleMinimize} title="Minimize">
      <Minus size={14} />
    </button>
    <button class="control-btn" onclick={handleMaximize} title={isMaximized ? "Restore" : "Maximize"}>
      {#if isMaximized}
        <Copy size={12} />
      {:else}
        <Square size={12} />
      {/if}
    </button>
    <button class="control-btn close-btn" onclick={handleClose} title="Close">
      <X size={14} />
    </button>
  </div>
</div>

<style>
  .titlebar {
    height: 36px;
    min-height: 36px;
    background-color: var(--bg-primary);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 8px;
    user-select: none;
    -webkit-user-select: none;
    border-bottom: 1px solid var(--bg-tertiary);
  }

  .titlebar-content {
    display: flex;
    align-items: center;
    gap: 8px;
    flex: 1;
  }

  .titlebar-icon {
    width: 20px;
    height: 20px;
    border-radius: 4px;
    pointer-events: none;
  }

  .titlebar-title {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-secondary);
    pointer-events: none;
  }

  .window-controls {
    display: flex;
    align-items: center;
    gap: 2px;
  }

  .control-btn {
    width: 36px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    border-radius: 4px;
    color: var(--text-muted);
    cursor: pointer;
    transition: all 100ms ease;
  }

  .control-btn:hover {
    background-color: var(--bg-hover);
    color: var(--text-primary);
  }

  .control-btn.close-btn:hover {
    background-color: #e81123;
    color: white;
  }

  .control-btn :global(svg) {
    pointer-events: none;
  }
</style>
