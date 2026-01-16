<script lang="ts">
  interface Props {
    enabled: boolean;
    onchange: (enabled: boolean) => void;
  }

  let { enabled, onchange }: Props = $props();
</script>

<button
  class="toggle"
  class:enabled
  onclick={() => onchange(!enabled)}
  role="switch"
  aria-checked={enabled}
>
  <div class="thumb"></div>
</button>

<style>
  .toggle {
    position: relative;
    width: 44px;
    min-width: 44px;
    height: 24px;
    border-radius: 9999px;
    border: none;
    background-color: #333333;
    cursor: pointer;
    transition: background-color 150ms ease;
    flex-shrink: 0;
  }

  .toggle.enabled {
    background-color: var(--accent-primary);
  }

  .thumb {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background-color: white;
    transition: transform 150ms ease;
  }

  .toggle.enabled .thumb {
    transform: translateX(20px);
  }
</style>
