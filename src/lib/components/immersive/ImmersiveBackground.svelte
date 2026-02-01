<script lang="ts">
  /**
   * Immersive Background
   *
   * Renders the blurred album artwork background for the immersive player.
   * Uses WebGL2 when available (with ambient motion), falls back to CSS blur.
   */

  import { onMount } from 'svelte';
  import {
    BUILD_IMMERSIVE_ENABLED,
    isWebGL2Available,
    isRuntimeEnabled,
  } from '$lib/immersive';

  interface Props {
    artwork: string;
    /** Enable ambient motion effect (WebGL2 only) */
    enableAmbient?: boolean;
    /** Ambient motion intensity 0-1 (WebGL2 only) */
    ambientIntensity?: number;
  }

  let { artwork, enableAmbient = true, ambientIntensity }: Props = $props();

  // Determine which renderer to use
  let useWebGL = $state(false);
  let useFallback = $state(false);
  let WebGLCanvas: typeof import('$lib/immersive/ImmersiveAmbientCanvas.svelte').default | null = $state(null);

  // Fallback state (CSS-based blur)
  let canvasRef: HTMLCanvasElement | undefined = $state();
  let isLoading = $state(true);
  let currentArtwork = $state('');

  // Check capabilities and load WebGL component if available
  onMount(async () => {
    if (BUILD_IMMERSIVE_ENABLED && isRuntimeEnabled() && isWebGL2Available()) {
      try {
        // Dynamic import - only loads if WebGL2 is available
        const module = await import('$lib/immersive/ImmersiveAmbientCanvas.svelte');
        WebGLCanvas = module.default;
        useWebGL = true;
        console.log('[ImmersiveBackground] Using WebGL2 renderer');
      } catch (e) {
        console.warn('[ImmersiveBackground] Failed to load WebGL canvas:', e);
        useFallback = true;
      }
    } else {
      useFallback = true;
      console.log('[ImmersiveBackground] Using CSS fallback');
    }
  });

  /**
   * Handle WebGL fallback (if WebGL init fails)
   */
  function handleWebGLFallback(): void {
    console.warn('[ImmersiveBackground] WebGL failed, switching to CSS fallback');
    useWebGL = false;
    useFallback = true;
  }

  // =====================================================
  // CSS Fallback Implementation (original code)
  // =====================================================

  async function generateBlurredBackground(imageUrl: string): Promise<void> {
    if (!canvasRef || !imageUrl) return;

    const ctx = canvasRef.getContext('2d');
    if (!ctx) return;

    const img = new Image();
    img.crossOrigin = 'anonymous';

    img.onload = () => {
      const size = 64;
      canvasRef!.width = size;
      canvasRef!.height = size;

      ctx.drawImage(img, 0, 0, size, size);

      const imageData = ctx.getImageData(0, 0, size, size);
      const data = imageData.data;

      for (let i = 0; i < data.length; i += 4) {
        const r = data[i];
        const g = data[i + 1];
        const b = data[i + 2];
        const avg = (r + g + b) / 3;

        const satFactor = 1.3;
        let newR = avg + (r - avg) * satFactor;
        let newG = avg + (g - avg) * satFactor;
        let newB = avg + (b - avg) * satFactor;

        data[i] = Math.min(255, Math.max(0, newR * 0.55));
        data[i + 1] = Math.min(255, Math.max(0, newG * 0.55));
        data[i + 2] = Math.min(255, Math.max(0, newB * 0.55));
      }

      ctx.putImageData(imageData, 0, 0);
      isLoading = false;
    };

    img.onerror = () => {
      isLoading = false;
    };

    img.src = imageUrl;
  }

  // Track artwork changes for fallback renderer
  $effect(() => {
    if (useFallback && artwork && artwork !== currentArtwork) {
      currentArtwork = artwork;
      isLoading = true;
      generateBlurredBackground(artwork);
    }
  });
</script>

<div class="immersive-background">
  {#if useWebGL && WebGLCanvas}
    <!-- WebGL2 Renderer with ambient motion -->
    <WebGLCanvas
      artworkUrl={artwork}
      enableMotion={enableAmbient}
      intensity={ambientIntensity}
      onFallback={handleWebGLFallback}
    />
  {:else if useFallback}
    <!-- CSS Fallback Renderer -->
    <div class="fallback-container" class:loading={isLoading}>
      <canvas
        bind:this={canvasRef}
        class="background-canvas"
        aria-hidden="true"
      ></canvas>
    </div>
  {:else}
    <!-- Loading state while determining renderer -->
    <div class="loading-placeholder"></div>
  {/if}

  <!-- Dark overlay for better contrast -->
  <div class="dark-overlay"></div>
</div>

<style>
  .immersive-background {
    position: absolute;
    inset: 0;
    overflow: hidden;
    z-index: 0;
    background-color: #0a0a0b;
  }

  /* CSS Fallback styles */
  .fallback-container {
    position: absolute;
    inset: 0;
  }

  .background-canvas {
    position: absolute;
    inset: -80px;
    width: calc(100% + 160px);
    height: calc(100% + 160px);
    image-rendering: auto;
    filter: blur(40px);
    transform: scale(1.15) translateZ(0);
    will-change: opacity;
    transition: opacity 500ms ease-out;
  }

  .loading .background-canvas {
    opacity: 0;
  }

  .loading-placeholder {
    position: absolute;
    inset: 0;
    background-color: #0a0a0b;
  }

  /* Dark overlay */
  .dark-overlay {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0.15);
    pointer-events: none;
    z-index: 1;
  }
</style>
