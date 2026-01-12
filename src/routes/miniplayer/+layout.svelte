<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    children: Snippet;
  }

  let { children }: Props = $props();
</script>

<!-- Minimal layout for MiniPlayer - no sidebar, titlebar, etc. -->
<div class="miniplayer-layout">
  <div class="miniplayer-frame">
    {@render children()}
  </div>
</div>

<style>
  :global(html, body) {
    margin: 0;
    padding: 0;
    background: #0f0f12 !important;
    overflow: hidden;
  }

  .miniplayer-layout {
    width: 100vw;
    height: 100vh;
    background: #0f0f12;
    overflow: hidden;
    padding: 4px;
    box-sizing: border-box;
  }

  .miniplayer-frame {
    width: 100%;
    height: 100%;
    background: #18181b;
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
  }
</style>
