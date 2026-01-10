<script lang="ts">
  import { Download, Check, Loader, AlertCircle } from 'lucide-svelte';

  type DownloadStatus = 'none' | 'queued' | 'downloading' | 'ready' | 'failed';

  interface Props {
    status?: DownloadStatus;
    progress?: number;
    size?: number;
    onDownload?: () => void;
    onRemove?: () => void;
  }

  let {
    status = 'none',
    progress = 0,
    size = 16,
    onDownload,
    onRemove
  }: Props = $props();

  function handleClick(e: MouseEvent) {
    e.stopPropagation();

    if (status === 'none' || status === 'failed') {
      onDownload?.();
    } else if (status === 'ready') {
      onRemove?.();
    }
  }

  const title = $derived(() => {
    switch (status) {
      case 'none': return 'Download for offline';
      case 'queued': return 'Queued for download';
      case 'downloading': return `Downloading ${progress}%`;
      case 'ready': return 'Downloaded (click to remove)';
      case 'failed': return 'Download failed (click to retry)';
      default: return '';
    }
  });
</script>

<button
  class="download-button"
  class:downloading={status === 'downloading' || status === 'queued'}
  class:ready={status === 'ready'}
  class:failed={status === 'failed'}
  onclick={handleClick}
  title={title()}
  disabled={status === 'queued'}
>
  {#if status === 'ready'}
    <Check {size} />
  {:else if status === 'downloading' || status === 'queued'}
    <div class="progress-ring" style="--progress: {progress}">
      <Loader {size} class="spinning" />
    </div>
  {:else if status === 'failed'}
    <AlertCircle {size} />
  {:else}
    <Download {size} />
  {/if}
</button>

<style>
  .download-button {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    border-radius: 4px;
    transition: all 150ms ease;
    padding: 0;
  }

  .download-button:hover:not(:disabled) {
    color: var(--text-primary);
    background: var(--bg-hover);
  }

  .download-button:disabled {
    cursor: default;
    opacity: 0.5;
  }

  .download-button.ready {
    color: var(--success, #22c55e);
  }

  .download-button.ready:hover {
    color: var(--error, #ef4444);
  }

  .download-button.failed {
    color: var(--error, #ef4444);
  }

  .download-button.downloading {
    color: var(--accent-primary);
  }

  .progress-ring {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .progress-ring :global(.spinning) {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }
</style>
