#version 300 es
/**
 * Ambient Post-Process Shader
 *
 * Displays a PRE-BLURRED texture with subtle ambient motion.
 * NO blur computation here - the texture is already blurred.
 *
 * Effects (all UV-based, no per-pixel processing):
 * - Slow UV drift
 * - Subtle zoom breathing
 * - Color intensity breathing
 */

precision mediump float;

in vec2 v_texCoord;
out vec4 fragColor;

uniform sampler2D u_texture;
uniform float u_time;
uniform float u_intensity;

void main() {
    vec2 uv = v_texCoord;
    vec2 center = vec2(0.5);

    // Slow UV drift
    float driftX = sin(u_time * 0.1) * 0.04 * u_intensity;
    float driftY = cos(u_time * 0.08) * 0.03 * u_intensity;
    uv += vec2(driftX, driftY);

    // Subtle zoom breathing
    float zoom = 1.0 + sin(u_time * 0.06) * 0.03 * u_intensity;
    uv = center + (uv - center) / zoom;

    // Clamp to valid range
    uv = clamp(uv, 0.0, 1.0);

    // Sample pre-blurred texture (no blur computation needed)
    vec4 color = texture(u_texture, uv);

    // Subtle brightness breathing
    float breath = 1.0 + sin(u_time * 0.12) * 0.06 * u_intensity;
    color.rgb *= breath;

    fragColor = color;
}
