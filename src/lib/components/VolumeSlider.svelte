<script lang="ts">
  interface Props {
    value: number;
    onchange: (value: number) => void;
    max?: number;
    showValue?: boolean;
  }

  let { value, onchange, max = 100, showValue = false }: Props = $props();

  let sliderRef: HTMLDivElement;
  let isDragging = $state(false);

  const percentage = $derived((value / max) * 100);

  function handleMouseDown(e: MouseEvent) {
    isDragging = true;
    updateValue(e);
  }

  function updateValue(e: MouseEvent) {
    if (sliderRef) {
      const rect = sliderRef.getBoundingClientRect();
      const pct = Math.max(0, Math.min(100, ((e.clientX - rect.left) / rect.width) * 100));
      onchange(Math.round((pct / 100) * max));
    }
  }

  function handleMouseMove(e: MouseEvent) {
    if (isDragging) updateValue(e);
  }

  function handleMouseUp() {
    isDragging = false;
  }

  $effect(() => {
    if (isDragging) {
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
      return () => {
        document.removeEventListener('mousemove', handleMouseMove);
        document.removeEventListener('mouseup', handleMouseUp);
      };
    }
  });
</script>

<div class="slider-container">
  <div
    class="slider"
    bind:this={sliderRef}
    onmousedown={handleMouseDown}
    role="slider"
    tabindex="0"
    aria-valuenow={value}
    aria-valuemin={0}
    aria-valuemax={max}
  >
    <div class="fill" style="width: {percentage}%"></div>
    <div class="thumb" style="left: {percentage}%"></div>
  </div>
  {#if showValue}
    <span class="value">{value}s</span>
  {/if}
</div>

<style>
  .slider-container {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .slider {
    flex: 1;
    height: 4px;
    background-color: #333333;
    border-radius: 9999px;
    position: relative;
    cursor: pointer;
  }

  .fill {
    height: 100%;
    background-color: var(--accent-primary);
    border-radius: 9999px;
  }

  .thumb {
    position: absolute;
    top: 50%;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background-color: white;
    transform: translate(-50%, -50%);
    opacity: 0;
    transition: opacity 150ms ease;
  }

  .slider:hover .thumb {
    opacity: 1;
  }

  .value {
    font-size: 14px;
    color: var(--text-muted);
    min-width: 32px;
    text-align: right;
  }
</style>
