<script lang="ts">
  import { generateBlurredBackground, extractDominantColors, createGradientFromColors } from '$lib/utils/imageBlur';

  interface Props {
    artwork: string;
    mode?: 'blur' | 'gradient' | 'solid';
  }

  let { artwork, mode = 'blur' }: Props = $props();

  let backgroundUrl = $state('');
  let useCanvasBlur = $state(false);
  let isLoading = $state(true);
  let currentArtwork = $state('');

  // Generate background when artwork changes
  $effect(() => {
    if (!artwork || artwork === currentArtwork) return;

    currentArtwork = artwork;
    isLoading = true;
    useCanvasBlur = false;

    if (mode === 'blur') {
      // Try canvas blur first (most efficient)
      generateBlurredBackground(artwork)
        .then((dataUrl) => {
          backgroundUrl = dataUrl;
          useCanvasBlur = true;
          isLoading = false;
        })
        .catch((err) => {
          // CORS error - fallback to CSS blur (works but uses more GPU)
          console.warn('[ImmersiveBackground] Canvas blur failed (CORS), using CSS blur fallback');
          backgroundUrl = artwork;
          useCanvasBlur = false;
          isLoading = false;
        });
    } else if (mode === 'gradient') {
      fallbackToGradient();
    } else {
      backgroundUrl = '';
      isLoading = false;
    }
  });

  async function fallbackToGradient() {
    try {
      const colors = await extractDominantColors(artwork);
      backgroundUrl = createGradientFromColors(colors);
      useCanvasBlur = true; // Gradient doesn't need CSS blur
    } catch {
      backgroundUrl = 'linear-gradient(135deg, #1a1a2e 0%, #16213e 100%)';
      useCanvasBlur = true;
    }
    isLoading = false;
  }
</script>

<div class="immersive-background" class:loading={isLoading}>
  <!-- Background: either pre-blurred canvas or CSS blur fallback -->
  {#if useCanvasBlur}
    <!-- Pre-computed blur (efficient) -->
    <div
      class="background-image canvas-blur"
      style="background-image: url({backgroundUrl});"
    ></div>
  {:else if backgroundUrl}
    <!-- CSS blur fallback (works with CORS-restricted images) -->
    <img
      src={backgroundUrl}
      alt=""
      class="background-image css-blur"
      aria-hidden="true"
    />
  {/if}

  <!-- Overlay for consistent darkness and vignette -->
  <div class="background-overlay"></div>
</div>

<style>
  .immersive-background {
    position: absolute;
    inset: 0;
    overflow: hidden;
    z-index: 0;
  }

  .background-image {
    position: absolute;
    inset: -40px; /* Overflow to avoid blur edge artifacts */
    transition: opacity 300ms ease-out;
  }

  /* Canvas pre-blurred version (efficient - no GPU filter) */
  .background-image.canvas-blur {
    background-size: cover;
    background-position: center;
    background-repeat: no-repeat;
    transform: scale(1.1);
  }

  /* CSS blur fallback for CORS-restricted images */
  .background-image.css-blur {
    width: calc(100% + 80px);
    height: calc(100% + 80px);
    object-fit: cover;
    filter: blur(60px) saturate(1.2) brightness(0.5);
    transform: scale(1.2);
  }

  .loading .background-image {
    opacity: 0;
  }

  .background-overlay {
    position: absolute;
    inset: 0;

    /* Vignette effect + ensure readability */
    background:
      radial-gradient(ellipse at center, transparent 0%, rgba(0, 0, 0, 0.3) 100%),
      linear-gradient(to bottom, rgba(0, 0, 0, 0.2) 0%, transparent 30%, transparent 70%, rgba(0, 0, 0, 0.4) 100%);
  }
</style>
