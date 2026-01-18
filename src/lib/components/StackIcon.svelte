<script lang="ts">
  import { Layers } from 'lucide-svelte';
  import { subscribe, getCurrentContext, getContextDisplayInfo } from '$lib/stores/playbackContextStore';
  import { subscribe as subscribePrefs, getCachedPreferences } from '$lib/stores/playbackPreferencesStore';
  
  interface Props {
    size?: number;
    class?: string;
    onClick?: () => void;
  }
  
  let { size = 16, class: className = '', onClick }: Props = $props();
  
  let context = $state(getCurrentContext());
  let displayInfo = $state(context ? getContextDisplayInfo() : null);
  let showIcon = $state(getCachedPreferences().show_context_icon);
  let isHovering = $state(false);

  function handleClick(e: MouseEvent) {
    e.stopPropagation();
    e.preventDefault();
    console.log('[StackIcon] Button clicked');
    console.log('[StackIcon] onClick prop:', onClick);
    console.log('[StackIcon] Current context:', context);
    if (onClick) {
      console.log('[StackIcon] Calling onClick callback');
      onClick();
    } else {
      console.warn('[StackIcon] No onClick callback provided');
    }
  }

  // Subscribe to context changes
  $effect(() => {
    const unsubscribe = subscribe(() => {
      const newContext = getCurrentContext();
      const newDisplayInfo = newContext ? getContextDisplayInfo() : null;
      console.log('[StackIcon] Context changed:', { newContext, newDisplayInfo });
      context = newContext;
      displayInfo = newDisplayInfo;
    });

    return () => {
      unsubscribe();
    };
  });

  // Subscribe to preferences changes
  $effect(() => {
    const unsubscribe = subscribePrefs(() => {
      showIcon = getCachedPreferences().show_context_icon;
      console.log('[StackIcon] Icon visibility:', showIcon);
    });

    return () => {
      unsubscribe();
    };
  });
</script>

{#if context && displayInfo && showIcon}
  <div
    class="stack-icon-container {className}"
    onmouseenter={() => isHovering = true}
    onmouseleave={() => isHovering = false}
  >
    <button
      class="stack-icon-wrapper"
      onclick={handleClick}
      type="button"
    >
      <Layers size={size} strokeWidth={2} />
    </button>

    {#if isHovering}
      <div class="context-tooltip">
        <div class="tooltip-label">Playing from</div>
        <div class="tooltip-value">{displayInfo}</div>
        <div class="tooltip-hint">Click to navigate</div>
      </div>
    {/if}
  </div>
{/if}

<style>
  .stack-icon-container {
    position: relative;
    display: inline-flex;
  }

  .stack-icon-wrapper {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: var(--text-secondary);
    opacity: 0.7;
    transition: all 0.2s;
    flex-shrink: 0;
    background: none;
    border: none;
    padding: 4px;
    border-radius: 4px;
    cursor: pointer;
  }
  
  .stack-icon-wrapper:hover {
    opacity: 1;
    background: rgba(255, 255, 255, 0.05);
    color: var(--text-primary);
  }

  .stack-icon-wrapper:active {
    transform: scale(0.95);
  }

  .context-tooltip {
    position: absolute;
    bottom: calc(100% + 8px);
    left: 50%;
    transform: translateX(-50%);
    background: rgba(24, 24, 28, 0.98);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 6px;
    padding: 8px 12px;
    min-width: 140px;
    max-width: 220px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4);
    z-index: 9999;
    animation: tooltip-appear 150ms ease;
    pointer-events: none;
    white-space: nowrap;
  }

  @keyframes tooltip-appear {
    from {
      opacity: 0;
      transform: translateX(-50%) translateY(4px);
    }
    to {
      opacity: 1;
      transform: translateX(-50%) translateY(0);
    }
  }

  .tooltip-label {
    font-size: 9px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: rgba(255, 255, 255, 0.4);
    margin-bottom: 4px;
  }

  .tooltip-value {
    font-size: 12px;
    font-weight: 500;
    color: white;
  }

  .tooltip-hint {
    font-size: 10px;
    color: rgba(255, 255, 255, 0.5);
    margin-top: 4px;
  }
</style>
