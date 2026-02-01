/**
 * Immersive Renderer - Lifecycle Manager
 *
 * This module owns the immersive rendering lifecycle:
 * - Capability detection
 * - Initialization / destruction
 * - State management
 * - Debug instrumentation
 *
 * UI components should use this as the single entry point.
 */

import { writable, derived, get } from 'svelte/store';
import {
  BUILD_IMMERSIVE_ENABLED,
  shouldImmersiveBeAvailable,
  getWebGL2Info,
  getConfig,
  setRuntimeEnabled as setConfigRuntimeEnabled,
} from './config';
import type {
  ImmersiveState,
  ImmersiveBackend,
  UnavailableReason,
  ImmersiveMetrics,
} from './types';

// ============================================================================
// State Store
// ============================================================================

const initialState: ImmersiveState = {
  active: false,
  backend: 'disabled',
  unavailableReason: BUILD_IMMERSIVE_ENABLED ? undefined : 'build-disabled',
};

/** Internal writable store */
const stateStore = writable<ImmersiveState>(initialState);

/** Public read-only store for immersive state */
export const immersiveState = { subscribe: stateStore.subscribe };

/** Derived store: is immersive currently active? */
export const isImmersiveActive = derived(stateStore, ($state) => $state.active);

/** Derived store: current backend */
export const immersiveBackend = derived(stateStore, ($state) => $state.backend);

/** Derived store: debug metrics (null if not active) */
export const immersiveMetrics = derived(stateStore, ($state) => $state.metrics ?? null);

// ============================================================================
// Metrics Tracking
// ============================================================================

let frameCount = 0;
let lastFpsUpdate = 0;
let currentFps = 0;
let lastFrameTime = 0;

/**
 * Update metrics from render loop.
 * Called by ImmersiveAmbientCanvas on each frame.
 */
export function updateMetrics(frameTimeMs: number): void {
  frameCount++;
  lastFrameTime = frameTimeMs;

  const now = performance.now();
  if (now - lastFpsUpdate >= 1000) {
    currentFps = frameCount;
    frameCount = 0;
    lastFpsUpdate = now;

    // Update store with new metrics
    stateStore.update((state) => {
      if (!state.active) return state;
      return {
        ...state,
        metrics: {
          ...state.metrics!,
          fps: currentFps,
          frameTimeMs: lastFrameTime,
        },
      };
    });
  }
}

/**
 * Set GPU info in metrics (called once during init).
 */
function setGpuInfo(renderer: string): void {
  stateStore.update((state) => ({
    ...state,
    metrics: {
      fps: 0,
      frameTimeMs: 0,
      gpuRenderer: renderer,
      textureCount: 0,
      gpuMemoryBytes: 0,
    },
  }));
}

/**
 * Update texture count in metrics.
 */
export function updateTextureCount(count: number, memoryBytes: number): void {
  stateStore.update((state) => {
    if (!state.metrics) return state;
    return {
      ...state,
      metrics: {
        ...state.metrics,
        textureCount: count,
        gpuMemoryBytes: memoryBytes,
      },
    };
  });
}

// ============================================================================
// Public API
// ============================================================================

/**
 * Check if immersive rendering is available.
 * This checks build flags, runtime preferences, and WebGL2 capability.
 */
export function isAvailable(): boolean {
  return shouldImmersiveBeAvailable().available;
}

/**
 * Check if immersive is currently enabled (user preference).
 */
export function isEnabled(): boolean {
  const { available, reason } = shouldImmersiveBeAvailable();
  if (!available && reason === 'runtime-disabled') {
    return false;
  }
  return available;
}

/**
 * Enable or disable immersive rendering (user preference).
 */
export function setEnabled(enabled: boolean): void {
  setConfigRuntimeEnabled(enabled);

  if (enabled) {
    // Check if we can actually enable
    const { available, reason } = shouldImmersiveBeAvailable();
    if (!available) {
      stateStore.set({
        active: false,
        backend: 'disabled',
        unavailableReason: reason,
      });
    }
  } else {
    stateStore.set({
      active: false,
      backend: 'disabled',
      unavailableReason: 'runtime-disabled',
    });
  }
}

/**
 * Initialize immersive rendering.
 * Call this when the immersive view is mounted.
 *
 * Returns true if initialization succeeded.
 */
export async function init(): Promise<boolean> {
  const { available, reason } = shouldImmersiveBeAvailable();

  if (!available) {
    console.log(`[Immersive] Not available: ${reason}`);
    stateStore.set({
      active: false,
      backend: 'disabled',
      unavailableReason: reason,
    });
    return false;
  }

  // Get GPU info for debugging
  const gpuInfo = getWebGL2Info();
  const renderer = gpuInfo?.renderer ?? 'Unknown GPU';
  console.log(`[Immersive] Initializing with WebGL2 - ${renderer}`);

  // Set initial state
  stateStore.set({
    active: true,
    backend: 'webgl2',
    metrics: {
      fps: 0,
      frameTimeMs: 0,
      gpuRenderer: renderer,
      textureCount: 0,
      gpuMemoryBytes: 0,
    },
  });

  return true;
}

/**
 * Destroy immersive rendering.
 * Call this when the immersive view is unmounted.
 */
export function destroy(): void {
  console.log('[Immersive] Destroyed');

  // Reset metrics
  frameCount = 0;
  lastFpsUpdate = 0;
  currentFps = 0;

  stateStore.set({
    active: false,
    backend: 'disabled',
  });
}

/**
 * Handle WebGL context loss.
 * Call this from the canvas component when context is lost.
 */
export function handleContextLost(): void {
  console.warn('[Immersive] WebGL context lost');
  stateStore.update((state) => ({
    ...state,
    active: false,
    backend: 'fallback',
    unavailableReason: 'context-lost',
  }));
}

/**
 * Handle WebGL context restored.
 * Call this from the canvas component when context is restored.
 */
export function handleContextRestored(): void {
  console.log('[Immersive] WebGL context restored');
  stateStore.update((state) => ({
    ...state,
    active: true,
    backend: 'webgl2',
    unavailableReason: undefined,
  }));
}

/**
 * Get current state snapshot (for debugging).
 */
export function getState(): ImmersiveState {
  return get(stateStore);
}

/**
 * Get current configuration.
 */
export { getConfig } from './config';
