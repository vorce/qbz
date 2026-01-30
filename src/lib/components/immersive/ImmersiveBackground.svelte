<script lang="ts">
  interface Props {
    artwork: string;
  }

  let { artwork }: Props = $props();

  let isLoading = $state(true);
  let currentArtwork = $state('');

  // Track artwork changes for fade transition
  $effect(() => {
    if (!artwork || artwork === currentArtwork) return;
    currentArtwork = artwork;
    isLoading = true;
  });

  function handleImageLoad() {
    isLoading = false;
  }
</script>

<div class="immersive-background" class:loading={isLoading}>
  <!-- Full artwork as background -->
  {#if artwork}
    <img
      src={artwork}
      alt=""
      class="background-image"
      aria-hidden="true"
      onload={handleImageLoad}
    />
  {/if}

  <!-- Vignette overlay - strong fade to black at edges -->
  <div class="vignette-overlay"></div>

  <!-- Gradient overlay for text readability -->
  <div class="gradient-overlay"></div>
</div>

<style>
  .immersive-background {
    position: absolute;
    inset: 0;
    overflow: hidden;
    z-index: 0;
    background-color: #0a0a0b;
  }

  .background-image {
    position: absolute;
    /* Scale larger than viewport to hide edges */
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%) scale(1.15);
    min-width: 115%;
    min-height: 115%;
    width: auto;
    height: auto;
    object-fit: cover;
    object-position: center;
    transition: opacity 400ms ease-out;
    /* Slight desaturation for better text contrast */
    filter: saturate(0.85) brightness(0.9);
  }

  .loading .background-image {
    opacity: 0;
  }

  /* Strong vignette - fades to black at edges */
  .vignette-overlay {
    position: absolute;
    inset: 0;
    background: radial-gradient(
      ellipse 80% 80% at 50% 45%,
      transparent 0%,
      transparent 30%,
      rgba(0, 0, 0, 0.4) 60%,
      rgba(0, 0, 0, 0.85) 100%
    );
    pointer-events: none;
  }

  /* Gradient for text areas - darker where text appears */
  .gradient-overlay {
    position: absolute;
    inset: 0;
    background:
      /* Top gradient for header */
      linear-gradient(to bottom, rgba(0, 0, 0, 0.5) 0%, transparent 15%),
      /* Bottom gradient for controls */
      linear-gradient(to top, rgba(0, 0, 0, 0.7) 0%, transparent 25%),
      /* Right side gradient for lyrics panel */
      linear-gradient(to left, rgba(0, 0, 0, 0.4) 0%, transparent 40%);
    pointer-events: none;
  }
</style>
