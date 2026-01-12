<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    icon: Snippet;
    label: string;
    badge?: string;
    active?: boolean;
    onclick?: () => void;
    class?: string;
  }

  let { icon, label, badge, active = false, onclick, class: className = '' }: Props = $props();
</script>

<button
  {onclick}
  class="nav-item {className}"
  class:active
  title={label}
>
  <div class="icon-container">
    {@render icon()}
  </div>
  <span class="label">{label}</span>
  {#if badge}
    <span class="badge">{badge}</span>
  {/if}
</button>

<style>
  .nav-item {
    width: 100%;
    height: 32px;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 8px;
    border-radius: 6px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    transition: all 150ms ease;
    text-align: left;
  }

  .nav-item:hover {
    background-color: var(--bg-hover);
  }

  .nav-item.active {
    background-color: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .icon-container {
    width: 14px;
    height: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .label {
    font-size: 13px;
    font-weight: 400;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    flex: 1;
    min-width: 0;
  }

  .badge {
    font-size: 11px;
    color: var(--text-muted);
    flex-shrink: 0;
  }
</style>
