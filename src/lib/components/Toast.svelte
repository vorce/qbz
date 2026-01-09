<script lang="ts">
  import { onMount } from 'svelte';
  import { CheckCircle, AlertCircle, Info, X } from 'lucide-svelte';

  interface Props {
    message: string;
    type?: 'success' | 'error' | 'info';
    onClose: () => void;
  }

  let { message, type = 'success', onClose }: Props = $props();

  onMount(() => {
    const timer = setTimeout(onClose, 4000);
    return () => clearTimeout(timer);
  });
</script>

<div class="toast" class:success={type === 'success'} class:error={type === 'error'} class:info={type === 'info'}>
  <div class="icon">
    {#if type === 'success'}
      <CheckCircle size={20} />
    {:else if type === 'error'}
      <AlertCircle size={20} />
    {:else}
      <Info size={20} />
    {/if}
  </div>
  <span class="message">{message}</span>
  <button class="close-btn" onclick={onClose}>
    <X size={16} />
  </button>
</div>

<style>
  .toast {
    position: fixed;
    bottom: 100px;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    background-color: var(--bg-tertiary);
    border-radius: 8px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4);
    z-index: 100;
    animation: slideUp 200ms ease-out;
  }

  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateX(-50%) translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateX(-50%) translateY(0);
    }
  }

  .icon {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .toast.success .icon {
    color: #4CAF50;
  }

  .toast.error .icon {
    color: #ff6b6b;
  }

  .toast.info .icon {
    color: var(--accent-primary);
  }

  .message {
    font-size: 14px;
    color: var(--text-primary);
  }

  .close-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: color 150ms ease;
  }

  .close-btn:hover {
    color: var(--text-primary);
  }
</style>
