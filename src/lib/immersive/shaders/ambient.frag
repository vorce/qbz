#version 300 es
/**
 * Ambient Post-Process Shader
 *
 * Displays a PRE-BLURRED texture with subtle ambient motion.
 * Additional GPU-side smoothing via multi-sample.
 *
 * Effects:
 * - Multi-sample blur (4 corners averaged)
 * - Luminance preservation over saturation
 * - Gradient tint (cool top, warm bottom)
 * - Slow UV drift
 * - Subtle zoom breathing
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

    // Multi-sample blur (4 corners averaged for extra smoothing)
    float offset = 0.01;
    vec3 c1 = texture(u_texture, uv + vec2( offset,  offset)).rgb;
    vec3 c2 = texture(u_texture, uv + vec2(-offset,  offset)).rgb;
    vec3 c3 = texture(u_texture, uv + vec2( offset, -offset)).rgb;
    vec3 c4 = texture(u_texture, uv + vec2(-offset, -offset)).rgb;
    vec3 color = (c1 + c2 + c3 + c4) * 0.25;

    // Preserve luminance over saturation
    float luma = dot(color, vec3(0.2126, 0.7152, 0.0722));
    // Avoid division by zero for very dark colors
    float len = length(color);
    if (len > 0.001) {
        color = normalize(color) * luma;
    }

    // Gradient tint (cool top, warm bottom) - anti-flat
    vec3 tintTop = vec3(0.95, 0.97, 1.0);    // Cool/blue
    vec3 tintBottom = vec3(1.0, 0.96, 0.92); // Warm/orange
    float t = v_texCoord.y; // Use original UV for gradient
    color *= mix(tintBottom, tintTop, t);

    // Subtle brightness breathing
    float breath = 1.0 + sin(u_time * 0.12) * 0.06 * u_intensity;
    color *= breath;

    fragColor = vec4(color, 1.0);
}
