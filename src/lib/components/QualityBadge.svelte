<script lang="ts">
  import { Disc } from 'lucide-svelte';

  interface Props {
    quality?: string;
    bitDepth?: number;
    samplingRate?: number;
    size?: 'sm' | 'md';
  }

  let {
    quality = '',
    bitDepth,
    samplingRate,
    size = 'sm'
  }: Props = $props();

  // Determine quality tier
  const tier = $derived.by(() => {
    if (bitDepth && bitDepth >= 24 && samplingRate && samplingRate > 96) {
      return 'max'; // Hi-Res+ (24-bit/>96kHz)
    }
    if (bitDepth && bitDepth >= 24) {
      return 'hires'; // Hi-Res (24-bit)
    }
    if (quality.toLowerCase().includes('flac') || quality.toLowerCase().includes('16-bit') || quality.toLowerCase().includes('lossless')) {
      return 'cd'; // CD Quality
    }
    if (quality.toLowerCase().includes('mp3')) {
      return 'lossy';
    }
    // Default based on sampling rate
    if (samplingRate && samplingRate >= 44.1) {
      if (bitDepth && bitDepth >= 24) return 'hires';
      return 'cd';
    }
    return 'unknown';
  });

  const sizeClass = $derived(size === 'md' ? 'badge-md' : 'badge-sm');
</script>

{#if tier === 'max'}
  <span class="badge badge-max {sizeClass}">MAX</span>
{:else if tier === 'hires'}
  <span class="badge badge-hires {sizeClass}">HR</span>
{:else if tier === 'cd'}
  <span class="badge badge-cd {sizeClass}">
    <Disc size={size === 'md' ? 12 : 10} />
  </span>
{/if}

<style>
  .badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    border-radius: 3px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .badge-sm {
    height: 16px;
    padding: 0 4px;
    font-size: 9px;
    gap: 2px;
  }

  .badge-md {
    height: 20px;
    padding: 0 6px;
    font-size: 10px;
    gap: 3px;
  }

  .badge-max {
    background: linear-gradient(135deg, #FFD700 0%, #FFA500 100%);
    color: #1a1a1a;
  }

  .badge-hires {
    background: linear-gradient(135deg, #4CAF50 0%, #2E7D32 100%);
    color: white;
  }

  .badge-cd {
    background: rgba(255, 255, 255, 0.15);
    color: var(--text-muted);
  }
</style>
