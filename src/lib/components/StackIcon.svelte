<script lang="ts">
  import { Layers } from 'lucide-svelte';
  import { subscribe, getCurrentContext, getContextDisplayInfo } from '$lib/stores/playbackContextStore';
  
  interface Props {
    size?: number;
    class?: string;
  }
  
  let { size = 16, class: className = '' }: Props = $props();
  
  let context = $state(getCurrentContext());
  let displayInfo = $state(context ? getContextDisplayInfo() : null);

  // Subscribe to context changes with $effect
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
</script>

{#if context && displayInfo}
  <div class="stack-icon-wrapper {className}" title="Playing from: {displayInfo}">
    <Layers size={size} strokeWidth={2} />
  </div>
{:else}
  <!-- Debug: Icon should be hidden -->
  <!-- <span style="font-size: 10px; color: red;">No context</span> -->
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
