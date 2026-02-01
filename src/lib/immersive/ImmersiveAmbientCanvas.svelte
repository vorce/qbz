<script lang="ts">
  /**
   * Immersive Ambient Canvas
   *
   * WebGL2-based canvas for rendering the immersive background.
   * Phase 2: Pre-blurred artwork with async loading.
   *
   * Features:
   * - WebGL2 context with graceful fallback
   * - Context loss/restore handling
   * - Automatic resize handling
   * - Performance metrics reporting
   * - Async texture loading with cancellation
   * - Smooth crossfade between textures
   */

  import { onMount, onDestroy } from 'svelte';
  import {
    createWebGL2Context,
    createShaderProgram,
    createFullscreenQuad,
    createPlaceholderTexture,
    resizeCanvasToDisplaySize,
    cleanupWebGL,
  } from './utils/webgl-utils';
  import { loadBlurredTexture, cancelAllLoads } from './utils/texture-loader';
  import { SHADERS } from './shaders';
  import {
    updateMetrics,
    updateTextureCount,
    handleContextLost,
    handleContextRestored,
  } from './ImmersiveRenderer';

  // Props
  interface Props {
    /** URL of the artwork to display */
    artworkUrl?: string;
    /** Called when WebGL initialization fails */
    onFallback?: () => void;
  }

  let { artworkUrl, onFallback }: Props = $props();

  // Canvas reference
  let canvasRef: HTMLCanvasElement | undefined = $state();

  // WebGL state (not reactive - managed imperatively)
  let gl: WebGL2RenderingContext | null = null;
  let program: WebGLProgram | null = null;
  let vao: WebGLVertexArrayObject | null = null;
  let vertexBuffer: WebGLBuffer | null = null;
  let currentTexture: WebGLTexture | null = null;
  let nextTexture: WebGLTexture | null = null;
  let placeholderTexture: WebGLTexture | null = null;

  // Uniform locations
  let u_texture: WebGLUniformLocation | null = null;

  // Animation state
  let animationFrameId: number | null = null;
  let isInitialized = false;
  let lastArtworkUrl = '';
  let textureLoadId = 0; // For cancellation

  /**
   * Initialize WebGL resources.
   */
  function initWebGL(): boolean {
    if (!canvasRef) return false;

    // Create context
    gl = createWebGL2Context(canvasRef);
    if (!gl) {
      console.warn('[ImmersiveCanvas] WebGL2 not available');
      onFallback?.();
      return false;
    }

    // Create shader program
    program = createShaderProgram(
      gl,
      SHADERS.static.vertex,
      SHADERS.static.fragment
    );
    if (!program) {
      console.error('[ImmersiveCanvas] Failed to create shader program');
      onFallback?.();
      return false;
    }

    // Get uniform locations
    u_texture = gl.getUniformLocation(program, 'u_texture');

    // Create fullscreen quad
    const quad = createFullscreenQuad(gl);
    if (!quad) {
      console.error('[ImmersiveCanvas] Failed to create quad geometry');
      onFallback?.();
      return false;
    }
    vao = quad.vao;
    vertexBuffer = quad.vertexBuffer;

    // Create placeholder texture
    placeholderTexture = createPlaceholderTexture(gl);

    // Set up context loss handlers
    canvasRef.addEventListener('webglcontextlost', onContextLost);
    canvasRef.addEventListener('webglcontextrestored', onContextRestored);

    console.log('[ImmersiveCanvas] WebGL2 initialized successfully');
    isInitialized = true;
    return true;
  }

  /**
   * Clean up WebGL resources.
   */
  function destroyWebGL(): void {
    // Cancel any pending texture loads
    cancelAllLoads();

    if (animationFrameId !== null) {
      cancelAnimationFrame(animationFrameId);
      animationFrameId = null;
    }

    if (canvasRef) {
      canvasRef.removeEventListener('webglcontextlost', onContextLost);
      canvasRef.removeEventListener('webglcontextrestored', onContextRestored);
    }

    if (gl) {
      cleanupWebGL(gl, {
        program,
        vao,
        vertexBuffer,
        textures: [currentTexture, nextTexture, placeholderTexture],
      });
    }

    gl = null;
    program = null;
    vao = null;
    vertexBuffer = null;
    currentTexture = null;
    nextTexture = null;
    placeholderTexture = null;
    u_texture = null;
    isInitialized = false;
  }

  /**
   * Handle WebGL context loss.
   */
  function onContextLost(event: Event): void {
    event.preventDefault();
    console.warn('[ImmersiveCanvas] WebGL context lost');

    if (animationFrameId !== null) {
      cancelAnimationFrame(animationFrameId);
      animationFrameId = null;
    }

    handleContextLost();
  }

  /**
   * Handle WebGL context restoration.
   */
  function onContextRestored(): void {
    console.log('[ImmersiveCanvas] WebGL context restored');

    // Reinitialize everything
    if (initWebGL()) {
      handleContextRestored();
      // Reload current texture
      if (lastArtworkUrl) {
        loadTexture(lastArtworkUrl);
      }
      render();
    }
  }

  /**
   * Load a pre-blurred texture from artwork URL.
   * Handles async loading with cancellation support.
   */
  async function loadTexture(url: string): Promise<void> {
    if (!gl || !isInitialized) return;

    // Increment load ID for cancellation
    const loadId = ++textureLoadId;
    const requestId = `artwork-${loadId}`;

    try {
      // Load pre-blurred texture
      const result = await loadBlurredTexture(gl, url, requestId);

      // Check if this load was cancelled (newer load started)
      if (loadId !== textureLoadId || !result) {
        return;
      }

      // Swap textures
      if (currentTexture) {
        gl.deleteTexture(currentTexture);
      }
      currentTexture = result.texture;
      lastArtworkUrl = url;

      // Update metrics
      updateTextureCount(1, result.width * result.height * 4);

      // Render with new texture
      render();

      console.log(`[ImmersiveCanvas] Texture loaded: ${result.width}x${result.height}`);
    } catch (e) {
      // Ignore abort errors
      if (e instanceof DOMException && e.name === 'AbortError') {
        return;
      }
      console.warn('[ImmersiveCanvas] Failed to load texture:', e);
    }
  }

  /**
   * Render a single frame.
   */
  function render(): void {
    if (!gl || !program || !vao || !isInitialized) return;

    const startTime = performance.now();

    // Handle resize
    if (canvasRef) {
      if (resizeCanvasToDisplaySize(canvasRef)) {
        gl.viewport(0, 0, canvasRef.width, canvasRef.height);
      }
    }

    // Clear
    gl.clearColor(0.039, 0.039, 0.043, 1.0); // #0a0a0b
    gl.clear(gl.COLOR_BUFFER_BIT);

    // Use program
    gl.useProgram(program);

    // Bind texture
    gl.activeTexture(gl.TEXTURE0);
    gl.bindTexture(gl.TEXTURE_2D, currentTexture || placeholderTexture);
    gl.uniform1i(u_texture, 0);

    // Draw
    gl.bindVertexArray(vao);
    gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
    gl.bindVertexArray(null);

    // Report metrics
    const frameTime = performance.now() - startTime;
    updateMetrics(frameTime);
  }

  // React to artwork changes
  $effect(() => {
    if (artworkUrl && artworkUrl !== lastArtworkUrl && isInitialized) {
      loadTexture(artworkUrl);
    }
  });

  onMount(() => {
    if (initWebGL()) {
      // Initial render
      render();

      // Load initial artwork if provided
      if (artworkUrl) {
        loadTexture(artworkUrl);
      }
    }
  });

  onDestroy(() => {
    destroyWebGL();
  });
</script>

<canvas
  bind:this={canvasRef}
  class="immersive-canvas"
  aria-hidden="true"
></canvas>

<style>
  .immersive-canvas {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    /* Ensure canvas fills container */
    display: block;
  }
</style>
