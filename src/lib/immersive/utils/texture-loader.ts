/**
 * Texture Loader
 *
 * Creates a high-quality pre-blurred texture that matches CSS blur(40px) quality.
 * The blur is computed ONCE at load time - no per-frame processing.
 *
 * Strategy:
 * 1. Load image at reasonable size (256x256)
 * 2. Apply heavy Gaussian-like blur via multiple canvas filter passes
 * 3. Apply color adjustments (saturation, brightness)
 * 4. Upload to GPU texture ONCE
 */

import { createTextureFromImage } from './webgl-utils';

// Cache of loaded blurred images
const blurCache = new Map<string, string>();
const MAX_CACHE_SIZE = 20;

// Active load requests (for cancellation)
const activeLoads = new Map<string, AbortController>();

/**
 * Apply multiple blur passes to achieve CSS blur(40px) equivalent.
 * Uses OffscreenCanvas if available for better performance.
 */
function applyHeavyBlur(
  sourceCanvas: HTMLCanvasElement,
  passes: number = 4,
  blurPerPass: number = 12
): HTMLCanvasElement {
  let current = sourceCanvas;
  const size = sourceCanvas.width;

  for (let i = 0; i < passes; i++) {
    const next = document.createElement('canvas');
    next.width = size;
    next.height = size;
    const ctx = next.getContext('2d');
    if (!ctx) return current;

    // Each pass applies blur
    ctx.filter = `blur(${blurPerPass}px)`;
    ctx.drawImage(current, 0, 0);

    current = next;
  }

  return current;
}

/**
 * Generate a pre-blurred background image.
 * This creates a texture that looks like CSS filter: blur(40px).
 */
async function generateBlurredImage(
  imageUrl: string,
  signal?: AbortSignal
): Promise<string> {
  // Check cache first
  const cached = blurCache.get(imageUrl);
  if (cached) return cached;

  return new Promise((resolve, reject) => {
    if (signal?.aborted) {
      reject(new DOMException('Aborted', 'AbortError'));
      return;
    }

    const img = new Image();
    img.crossOrigin = 'anonymous';

    const abortHandler = () => {
      img.src = '';
      reject(new DOMException('Aborted', 'AbortError'));
    };

    signal?.addEventListener('abort', abortHandler);

    img.onload = () => {
      signal?.removeEventListener('abort', abortHandler);

      if (signal?.aborted) {
        reject(new DOMException('Aborted', 'AbortError'));
        return;
      }

      try {
        // Step 1: Draw image to initial canvas at target size
        // 256x256 provides good quality while keeping processing fast
        const SIZE = 256;
        const BLUR_PASSES = 4;
        const BLUR_PER_PASS = 15; // 4 passes * 15px ≈ CSS blur(40-50px)

        const sourceCanvas = document.createElement('canvas');
        sourceCanvas.width = SIZE;
        sourceCanvas.height = SIZE;
        const sourceCtx = sourceCanvas.getContext('2d');
        if (!sourceCtx) {
          reject(new Error('Could not get canvas context'));
          return;
        }

        // Draw image scaled to fit canvas
        sourceCtx.drawImage(img, 0, 0, SIZE, SIZE);

        // Step 2: Apply heavy blur (multiple passes)
        const blurredCanvas = applyHeavyBlur(sourceCanvas, BLUR_PASSES, BLUR_PER_PASS);

        // Step 3: Apply color adjustments
        const finalCanvas = document.createElement('canvas');
        finalCanvas.width = SIZE;
        finalCanvas.height = SIZE;
        const finalCtx = finalCanvas.getContext('2d');
        if (!finalCtx) {
          reject(new Error('Could not get final canvas context'));
          return;
        }

        // Saturation boost + brightness reduction for background use
        finalCtx.filter = 'saturate(1.3) brightness(0.55)';
        finalCtx.drawImage(blurredCanvas, 0, 0);

        // Step 4: Convert to data URL
        const dataUrl = finalCanvas.toDataURL('image/jpeg', 0.85);

        // Cache the result
        if (blurCache.size >= MAX_CACHE_SIZE) {
          const firstKey = blurCache.keys().next().value;
          if (firstKey) blurCache.delete(firstKey);
        }
        blurCache.set(imageUrl, dataUrl);

        resolve(dataUrl);
      } catch (err) {
        reject(err);
      }
    };

    img.onerror = () => {
      signal?.removeEventListener('abort', abortHandler);
      reject(new Error(`Failed to load image: ${imageUrl}`));
    };

    img.src = imageUrl;
  });
}

/**
 * Load image from URL or data URL.
 */
function loadImage(src: string, signal?: AbortSignal): Promise<HTMLImageElement> {
  return new Promise((resolve, reject) => {
    if (signal?.aborted) {
      reject(new DOMException('Aborted', 'AbortError'));
      return;
    }

    const img = new Image();
    img.crossOrigin = 'anonymous';

    const abortHandler = () => {
      img.src = '';
      reject(new DOMException('Aborted', 'AbortError'));
    };

    signal?.addEventListener('abort', abortHandler);

    img.onload = () => {
      signal?.removeEventListener('abort', abortHandler);
      resolve(img);
    };

    img.onerror = () => {
      signal?.removeEventListener('abort', abortHandler);
      reject(new Error(`Failed to load image: ${src}`));
    };

    img.src = src;
  });
}

export interface LoadTextureResult {
  texture: WebGLTexture;
  width: number;
  height: number;
}

/**
 * Load a pre-blurred texture from artwork URL.
 *
 * The texture is blurred ONCE at load time.
 * No per-frame blur processing needed.
 */
export async function loadBlurredTexture(
  gl: WebGL2RenderingContext,
  artworkUrl: string,
  requestId: string = 'default'
): Promise<LoadTextureResult | null> {
  // Cancel any previous load with the same request ID
  const existingController = activeLoads.get(requestId);
  if (existingController) {
    existingController.abort();
  }

  const controller = new AbortController();
  activeLoads.set(requestId, controller);

  try {
    // Generate pre-blurred image (cached)
    const blurredDataUrl = await generateBlurredImage(
      artworkUrl,
      controller.signal
    );

    // Load as image element
    const img = await loadImage(blurredDataUrl, controller.signal);

    // Create texture (uploaded ONCE)
    const texture = createTextureFromImage(gl, img);
    if (!texture) {
      throw new Error('Failed to create texture');
    }

    activeLoads.delete(requestId);

    return {
      texture,
      width: img.width,
      height: img.height,
    };
  } catch (err) {
    activeLoads.delete(requestId);

    if (err instanceof DOMException && err.name === 'AbortError') {
      return null;
    }

    console.warn('[TextureLoader] Failed to load texture:', err);
    return null;
  }
}

/**
 * Cancel all active texture loads.
 */
export function cancelAllLoads(): void {
  for (const controller of activeLoads.values()) {
    controller.abort();
  }
  activeLoads.clear();
}

/**
 * Clear the blur cache.
 */
export function clearBlurCache(): void {
  blurCache.clear();
}

/**
 * Get cache statistics.
 */
export function getBlurCacheStats(): { size: number; maxSize: number } {
  return {
    size: blurCache.size,
    maxSize: MAX_CACHE_SIZE,
  };
}
