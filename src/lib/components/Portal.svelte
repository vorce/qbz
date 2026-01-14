<script lang="ts">
  import { onMount } from 'svelte';

  export let target: HTMLElement;

  let container: HTMLDivElement | null = null;

  onMount(() => {
    if (!container || !target) return;
    target.appendChild(container);
    return () => {
      if (container?.parentNode) {
        container.parentNode.removeChild(container);
      }
    };
  });
</script>

<div bind:this={container} class="portal-root">
  <slot />
</div>

<style>
  .portal-root {
    display: contents;
  }
</style>
