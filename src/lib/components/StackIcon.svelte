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
  <button
    class="stack-icon-wrapper {className}"
    title="Click to go to: {displayInfo}"
    onclick={handleClick}
    type="button"
  >
    <Layers size={size} strokeWidth={2} />
  </button>
{/if}

<style>
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
</style>
