<script lang="ts">
  import { Layers } from 'lucide-svelte';
  import { subscribe, getCurrentContext, getContextDisplayInfo } from '$lib/stores/playbackContextStore';
  import { subscribe as subscribePrefs, getCachedPreferences } from '$lib/stores/playbackPreferencesStore';
  
  interface Props {
    size?: number;
    class?: string;
  }
  
  let { size = 16, class: className = '' }: Props = $props();
  
  let context = $state(getCurrentContext());
  let displayInfo = $state(context ? getContextDisplayInfo() : null);
  let showIcon = $state(getCachedPreferences().show_context_icon);

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
  <div class="stack-icon-wrapper {className}" title="Playing from: {displayInfo}">
    <Layers size={size} strokeWidth={2} />
  </div>
{/if}

<style>
  .stack-icon-wrapper {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: var(--text-secondary);
    opacity: 0.7;
    transition: opacity 0.2s;
    flex-shrink: 0;
  }
  
  .stack-icon-wrapper:hover {
    opacity: 1;
  }
</style>
