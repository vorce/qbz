<script lang="ts">
  import { Layers } from 'lucide-svelte';
  import { subscribe, getCurrentContext, getContextDisplayInfo } from '$lib/stores/playbackContextStore';
  import { onMount, onDestroy } from 'svelte';
  
  interface Props {
    size?: number;
    class?: string;
  }
  
  let { size = 16, class: className = '' }: Props = $props();
  
  let context = $state(getCurrentContext());
  let displayInfo = $state(context ? getContextDisplayInfo() : null);

  let unsubscribe: (() => void) | null = null;

  onMount(() => {
    unsubscribe = subscribe(() => {
      context = getCurrentContext();
      displayInfo = context ? getContextDisplayInfo() : null;
    });
  });

  onDestroy(() => {
    unsubscribe?.();
  });
</script>

{#if context && displayInfo}
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
