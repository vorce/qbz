<script lang="ts">
  import { onMount } from 'svelte';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api/core';

  interface Props {
    enabled?: boolean;
    artwork?: string;
    trackTitle?: string;
    artist?: string;
  }

  let { enabled = true, artwork = '', trackTitle = '', artist = '' }: Props = $props();

  let canvasRef: HTMLCanvasElement | null = $state(null);
  let gl: WebGL2RenderingContext | null = null;
  let program: WebGLProgram | null = null;
  let frequencyTexture: WebGLTexture | null = null;
  let vao: WebGLVertexArrayObject | null = null;
  let animationFrame: number | null = null;
  let unlisten: UnlistenFn | null = null;
  let isInitialized = false;

  const NUM_BARS = 64;
  const frequencyData = new Float32Array(NUM_BARS);

  // Vertex shader - MIRROR mode: bars extend from center up and down
  const vertexSource = `#version 300 es
    precision highp float;

    in vec2 a_position;
    in float a_mirror; // 0 = top half, 1 = bottom half (mirrored)

    uniform sampler2D u_frequencies;
    uniform vec2 u_resolution;
    uniform float u_barWidth;
    uniform float u_gap;
    uniform float u_maxHeight;
    uniform float u_centerY;

    out float v_height;
    out float v_barIndex;
    out float v_verticalPos;

    void main() {
      int barIndex = gl_InstanceID / 2; // Each bar has 2 instances (top + bottom)
      int isBottom = gl_InstanceID % 2;

      float freq = texelFetch(u_frequencies, ivec2(barIndex, 0), 0).r;

      float totalWidth = u_barWidth + u_gap;
      float totalBarsWidth = float(${NUM_BARS}) * totalWidth - u_gap;
      float offsetX = (u_resolution.x - totalBarsWidth) / 2.0;

      float x = offsetX + float(barIndex) * totalWidth + a_position.x * u_barWidth;

      // Mirror: top bars go up from center, bottom bars go down
      float barHeight = freq * u_maxHeight;
      float y;
      if (isBottom == 0) {
        // Top half: y goes from centerY to centerY + height
        y = u_centerY + a_position.y * barHeight;
      } else {
        // Bottom half: y goes from centerY to centerY - height (mirrored)
        y = u_centerY - a_position.y * barHeight;
      }

      vec2 clipSpace = (vec2(x, y) / u_resolution) * 2.0 - 1.0;
      gl_Position = vec4(clipSpace * vec2(1, 1), 0, 1);

      v_height = freq;
      v_barIndex = float(barIndex) / float(${NUM_BARS});
      v_verticalPos = a_position.y;
    }
  `;

  // Fragment shader with enhanced visuals
  const fragmentSource = `#version 300 es
    precision highp float;

    in float v_height;
    in float v_barIndex;
    in float v_verticalPos;

    out vec4 fragColor;

    uniform vec3 u_colorLow;
    uniform vec3 u_colorMid;
    uniform vec3 u_colorHigh;

    void main() {
      // Three-color gradient based on vertical position
      vec3 color;
      if (v_verticalPos < 0.5) {
        color = mix(u_colorLow, u_colorMid, v_verticalPos * 2.0);
      } else {
        color = mix(u_colorMid, u_colorHigh, (v_verticalPos - 0.5) * 2.0);
      }

      // Intensity boost based on frequency height
      color *= 0.7 + 0.5 * v_height;

      // Glow effect for peaks
      float glow = smoothstep(0.5, 1.0, v_height) * 0.3;
      color += glow;

      // Slight transparency fade at edges
      float alpha = 0.9 - 0.1 * v_verticalPos;

      fragColor = vec4(color, alpha);
    }
  `;

  function createShader(glCtx: WebGL2RenderingContext, type: number, source: string): WebGLShader | null {
    const shader = glCtx.createShader(type);
    if (!shader) return null;

    glCtx.shaderSource(shader, source);
    glCtx.compileShader(shader);

    if (!glCtx.getShaderParameter(shader, glCtx.COMPILE_STATUS)) {
      console.error('Shader compile error:', glCtx.getShaderInfoLog(shader));
      glCtx.deleteShader(shader);
      return null;
    }

    return shader;
  }

  function createProgram(glCtx: WebGL2RenderingContext, vs: WebGLShader, fs: WebGLShader): WebGLProgram | null {
    const prog = glCtx.createProgram();
    if (!prog) return null;

    glCtx.attachShader(prog, vs);
    glCtx.attachShader(prog, fs);
    glCtx.linkProgram(prog);

    if (!glCtx.getProgramParameter(prog, glCtx.LINK_STATUS)) {
      console.error('Program link error:', glCtx.getProgramInfoLog(prog));
      glCtx.deleteProgram(prog);
      return null;
    }

    return prog;
  }

  async function initWebGL() {
    if (!canvasRef || isInitialized) return;

    gl = canvasRef.getContext('webgl2', { alpha: true, premultipliedAlpha: false });
    if (!gl) {
      console.warn('WebGL2 not available for visualizer');
      return;
    }

    const vs = createShader(gl, gl.VERTEX_SHADER, vertexSource);
    const fs = createShader(gl, gl.FRAGMENT_SHADER, fragmentSource);
    if (!vs || !fs) return;

    program = createProgram(gl, vs, fs);
    if (!program) return;

    // Create VAO with quad vertices
    vao = gl.createVertexArray();
    gl.bindVertexArray(vao);

    // Quad: bottom-left origin, extends up and right
    const positions = new Float32Array([
      0, 0,  1, 0,  0, 1,
      0, 1,  1, 0,  1, 1,
    ]);

    const posBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, posBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, positions, gl.STATIC_DRAW);

    const posLoc = gl.getAttribLocation(program, 'a_position');
    gl.enableVertexAttribArray(posLoc);
    gl.vertexAttribPointer(posLoc, 2, gl.FLOAT, false, 0, 0);

    // Create frequency texture
    frequencyTexture = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, frequencyTexture);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.R32F, NUM_BARS, 1, 0, gl.RED, gl.FLOAT, frequencyData);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);

    isInitialized = true;

    // Enable backend
    try {
      await invoke('set_visualizer_enabled', { enabled: true });
      console.log('[Visualizer] Backend enabled');
    } catch (e) {
      console.error('[Visualizer] Failed to enable backend:', e);
    }

    // Listen for frequency data
    unlisten = await listen<number[]>('viz:data', (event) => {
      const payload = event.payload;
      if (Array.isArray(payload)) {
        // Binary data came as array of numbers (bytes)
        const bytes = new Uint8Array(payload);
        const floats = new Float32Array(bytes.buffer);
        if (floats.length === NUM_BARS) {
          frequencyData.set(floats);
          updateFrequencyTexture();
        }
      }
    });

    render();
  }

  function updateFrequencyTexture() {
    if (!gl || !frequencyTexture) return;
    gl.bindTexture(gl.TEXTURE_2D, frequencyTexture);
    gl.texSubImage2D(gl.TEXTURE_2D, 0, 0, 0, NUM_BARS, 1, gl.RED, gl.FLOAT, frequencyData);
  }

  function render() {
    if (!gl || !program || !frequencyTexture || !canvasRef || !vao) return;

    const rect = canvasRef.getBoundingClientRect();
    const dpr = window.devicePixelRatio || 1;
    canvasRef.width = rect.width * dpr;
    canvasRef.height = rect.height * dpr;

    gl.viewport(0, 0, canvasRef.width, canvasRef.height);
    gl.clearColor(0, 0, 0, 0);
    gl.clear(gl.COLOR_BUFFER_BIT);

    gl.useProgram(program);
    gl.bindVertexArray(vao);

    // Calculate dimensions
    const barWidth = (rect.width / NUM_BARS) * 0.65;
    const gap = (rect.width / NUM_BARS) * 0.35;
    const maxHeight = rect.height * 0.35; // Each half gets 35% of height
    const centerY = rect.height * 0.5;

    // Set uniforms
    gl.uniform2f(gl.getUniformLocation(program, 'u_resolution'), rect.width, rect.height);
    gl.uniform1f(gl.getUniformLocation(program, 'u_barWidth'), barWidth);
    gl.uniform1f(gl.getUniformLocation(program, 'u_gap'), gap);
    gl.uniform1f(gl.getUniformLocation(program, 'u_maxHeight'), maxHeight);
    gl.uniform1f(gl.getUniformLocation(program, 'u_centerY'), centerY);

    // Colors: teal -> purple -> pink
    gl.uniform3f(gl.getUniformLocation(program, 'u_colorLow'), 0.0, 0.7, 0.7);
    gl.uniform3f(gl.getUniformLocation(program, 'u_colorMid'), 0.5, 0.2, 0.8);
    gl.uniform3f(gl.getUniformLocation(program, 'u_colorHigh'), 0.9, 0.3, 0.6);

    // Bind frequency texture
    gl.activeTexture(gl.TEXTURE0);
    gl.bindTexture(gl.TEXTURE_2D, frequencyTexture);
    gl.uniform1i(gl.getUniformLocation(program, 'u_frequencies'), 0);

    // Enable blending
    gl.enable(gl.BLEND);
    gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);

    // Draw: 2 instances per bar (top + bottom mirror)
    gl.drawArraysInstanced(gl.TRIANGLES, 0, 6, NUM_BARS * 2);

    animationFrame = requestAnimationFrame(render);
  }

  async function cleanup() {
    if (animationFrame) {
      cancelAnimationFrame(animationFrame);
      animationFrame = null;
    }

    if (unlisten) {
      unlisten();
      unlisten = null;
    }

    try {
      await invoke('set_visualizer_enabled', { enabled: false });
      console.log('[Visualizer] Backend disabled');
    } catch (e) {
      console.error('[Visualizer] Failed to disable backend:', e);
    }

    if (gl) {
      if (frequencyTexture) gl.deleteTexture(frequencyTexture);
      if (program) gl.deleteProgram(program);
      if (vao) gl.deleteVertexArray(vao);
    }

    isInitialized = false;
  }

  onMount(() => {
    if (enabled) {
      initWebGL();
    }
    return cleanup;
  });

  $effect(() => {
    if (enabled && !isInitialized) {
      initWebGL();
    } else if (!enabled && isInitialized) {
      cleanup();
    }
  });
</script>

<div class="visualizer-panel" class:visible={enabled}>
  <canvas bind:this={canvasRef} class="visualizer-canvas"></canvas>

  <!-- Track info overlay -->
  <div class="track-info">
    {#if artwork}
      <img src={artwork} alt={trackTitle} class="artwork" />
    {/if}
    <div class="text">
      <h1 class="title">{trackTitle}</h1>
      <p class="artist">{artist}</p>
    </div>
  </div>
</div>

<style>
  .visualizer-panel {
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    opacity: 0;
    transition: opacity 300ms ease;
    z-index: 5;
  }

  .visualizer-panel.visible {
    opacity: 1;
  }

  .visualizer-canvas {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
  }

  .track-info {
    position: relative;
    z-index: 10;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    gap: 16px;
  }

  .artwork {
    width: min(200px, 25vw);
    height: min(200px, 25vw);
    border-radius: 12px;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.5);
    object-fit: cover;
  }

  .text {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .title {
    font-size: clamp(18px, 2.5vw, 28px);
    font-weight: 700;
    color: white;
    margin: 0;
    text-shadow: 0 2px 10px rgba(0, 0, 0, 0.5);
  }

  .artist {
    font-size: clamp(14px, 1.8vw, 18px);
    color: rgba(255, 255, 255, 0.8);
    margin: 0;
    text-shadow: 0 2px 8px rgba(0, 0, 0, 0.5);
  }
</style>
