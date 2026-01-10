<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount, onDestroy } from 'svelte';
  import { X, Cast, Loader2, Monitor, Wifi } from 'lucide-svelte';

  interface Props {
    isOpen: boolean;
    onClose: () => void;
    onConnect: (deviceId: string) => void;
  }

  interface CastDevice {
    id: string;
    name: string;
    ip: string;
    port: number;
  }

  let { isOpen, onClose, onConnect }: Props = $props();

  let devices = $state<CastDevice[]>([]);
  let loading = $state(false);
  let error = $state<string | null>(null);
  let discoveryStarted = $state(false);

  onMount(() => {
    if (isOpen) {
      startDiscovery();
    }
  });

  onDestroy(() => {
    if (discoveryStarted) {
      stopDiscovery();
    }
  });

  $effect(() => {
    if (isOpen && !discoveryStarted) {
      startDiscovery();
    } else if (!isOpen && discoveryStarted) {
      stopDiscovery();
    }
  });

  async function startDiscovery() {
    loading = true;
    error = null;
    discoveryStarted = true;

    try {
      await invoke('cast_start_discovery');
      // Poll for devices
      pollDevices();
    } catch (err) {
      error = String(err);
      loading = false;
    }
  }

  async function stopDiscovery() {
    discoveryStarted = false;
    try {
      await invoke('cast_stop_discovery');
    } catch (err) {
      console.error('Failed to stop discovery:', err);
    }
  }

  async function pollDevices() {
    if (!discoveryStarted) return;

    try {
      const result = await invoke<CastDevice[]>('cast_get_devices');
      devices = result;
    } catch (err) {
      console.error('Failed to get devices:', err);
    }

    loading = false;

    // Continue polling while open
    if (discoveryStarted) {
      setTimeout(pollDevices, 2000);
    }
  }

  async function handleConnect(device: CastDevice) {
    try {
      await invoke('cast_connect', { deviceId: device.id });
      onConnect(device.id);
      onClose();
    } catch (err) {
      error = String(err);
    }
  }
</script>

{#if isOpen}
  <div class="overlay" onclick={onClose} role="presentation">
    <div class="picker" onclick={(e) => e.stopPropagation()} role="dialog">
      <div class="header">
        <h3>Cast to Device</h3>
        <button class="close-btn" onclick={onClose}>
          <X size={20} />
        </button>
      </div>

      <div class="content">
        {#if loading && devices.length === 0}
          <div class="loading">
            <Loader2 size={32} class="spin" />
            <p>Searching for devices...</p>
          </div>
        {:else if error}
          <div class="error">
            <p>{error}</p>
          </div>
        {:else if devices.length === 0}
          <div class="empty">
            <Wifi size={32} />
            <p>No devices found</p>
            <p class="hint">Make sure your Chromecast is on the same network</p>
          </div>
        {:else}
          <div class="devices">
            {#each devices as device}
              <button class="device" onclick={() => handleConnect(device)}>
                <Monitor size={24} />
                <div class="device-info">
                  <span class="device-name">{device.name}</span>
                  <span class="device-ip">{device.ip}</span>
                </div>
                <Cast size={20} class="cast-icon" />
              </button>
            {/each}
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    z-index: 200;
    background-color: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .picker {
    width: 360px;
    max-height: 400px;
    background-color: var(--bg-secondary);
    border-radius: 12px;
    overflow: hidden;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
  }

  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-bottom: 1px solid var(--bg-tertiary);
  }

  .header h3 {
    font-size: 16px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .close-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
  }

  .close-btn:hover {
    color: var(--text-primary);
    background-color: rgba(255, 255, 255, 0.1);
  }

  .content {
    padding: 16px;
    max-height: 300px;
    overflow-y: auto;
  }

  .loading, .empty, .error {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 32px;
    gap: 12px;
    color: var(--text-muted);
  }

  .loading :global(.spin) {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .hint {
    font-size: 12px;
    color: #666666;
  }

  .error {
    color: #ff6b6b;
  }

  .devices {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .device {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    background: none;
    border: 1px solid var(--bg-tertiary);
    border-radius: 8px;
    cursor: pointer;
    transition: all 150ms ease;
    text-align: left;
    width: 100%;
    color: var(--text-primary);
  }

  .device:hover {
    background-color: rgba(255, 255, 255, 0.05);
    border-color: var(--accent-primary);
  }

  .device-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .device-name {
    font-size: 14px;
    font-weight: 500;
  }

  .device-ip {
    font-size: 12px;
    color: var(--text-muted);
  }

  .device :global(.cast-icon) {
    color: var(--text-muted);
    opacity: 0;
    transition: opacity 150ms ease;
  }

  .device:hover :global(.cast-icon) {
    opacity: 1;
    color: var(--accent-primary);
  }
</style>
