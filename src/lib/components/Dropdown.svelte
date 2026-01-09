<script lang="ts">
  import { ChevronDown } from 'lucide-svelte';

  interface Props {
    value: string;
    options: string[];
    onchange: (value: string) => void;
  }

  let { value, options, onchange }: Props = $props();

  let isOpen = $state(false);
  let dropdownRef: HTMLDivElement;

  function handleClickOutside(event: MouseEvent) {
    if (dropdownRef && !dropdownRef.contains(event.target as Node)) {
      isOpen = false;
    }
  }

  $effect(() => {
    if (isOpen) {
      document.addEventListener('mousedown', handleClickOutside);
      return () => document.removeEventListener('mousedown', handleClickOutside);
    }
  });
</script>

<div class="dropdown" bind:this={dropdownRef}>
  <button class="trigger" onclick={() => (isOpen = !isOpen)}>
    <span>{value}</span>
    <ChevronDown size={16} class="chevron" />
  </button>

  {#if isOpen}
    <div class="menu">
      {#each options as option}
        <button
          class="option"
          class:selected={option === value}
          onclick={() => {
            onchange(option);
            isOpen = false;
          }}
        >
          {option}
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .dropdown {
    position: relative;
  }

  .trigger {
    height: 40px;
    min-width: 160px;
    padding: 0 16px;
    background-color: var(--bg-tertiary);
    border-radius: 8px;
    border: none;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    font-size: 14px;
    color: var(--text-primary);
    cursor: pointer;
    transition: background-color 150ms ease;
  }

  .trigger:hover {
    background-color: #333333;
  }

  .trigger :global(.chevron) {
    color: var(--text-muted);
  }

  .menu {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    margin-top: 4px;
    background-color: var(--bg-tertiary);
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4);
    z-index: 50;
  }

  .option {
    width: 100%;
    padding: 12px 16px;
    text-align: left;
    font-size: 14px;
    color: var(--text-secondary);
    background: none;
    border: none;
    cursor: pointer;
    transition: background-color 150ms ease;
  }

  .option:hover {
    background-color: var(--bg-hover);
  }

  .option.selected {
    background-color: #333333;
    color: var(--text-primary);
  }
</style>
