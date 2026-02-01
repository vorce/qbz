#version 300 es
/**
 * Static Texture Fragment Shader
 *
 * Simply samples a texture and outputs it.
 * No animation, no effects - just display the texture.
 *
 * Used as Phase 1 baseline before adding ambient motion.
 */

precision mediump float;

// Input: interpolated texture coordinates from vertex shader
in vec2 v_texCoord;

// Output: final fragment color
out vec4 fragColor;

// Uniforms
uniform sampler2D u_texture;

void main() {
    fragColor = texture(u_texture, v_texCoord);
}
