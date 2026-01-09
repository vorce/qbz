<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    icon: Snippet;
    label: string;
    active?: boolean;
    onclick?: () => void;
  }

  let { icon, label, active = false, onclick }: Props = $props();
</script>

<button
  {onclick}
  class="nav-item"
  class:active
>
  <div class="icon-container">
    {@render icon()}
  </div>
  <span class="label">{label}</span>
</button>

<style>
  .nav-item {
    width: 100%;
    height: 36px;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 12px;
    border-radius: 8px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    transition: all 150ms ease;
  }

  .nav-item:hover {
    background-color: var(--bg-hover);
  }

  .nav-item.active {
    background-color: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .icon-container {
    width: 18px;
    height: 18px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .label {
    font-size: 14px;
    font-weight: 400;
  }
</style>
