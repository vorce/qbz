#version 300 es
/**
 * Fullscreen Quad Vertex Shader
 *
 * Renders a quad that covers the entire screen.
 * Expects 4 vertices: (-1,-1), (1,-1), (-1,1), (1,1)
 * Drawn as TRIANGLE_STRIP.
 */

// Input: vertex position in clip space
layout(location = 0) in vec2 a_position;

// Output: texture coordinates for fragment shader
out vec2 v_texCoord;

void main() {
    // Convert from clip space (-1 to 1) to texture space (0 to 1)
    v_texCoord = a_position * 0.5 + 0.5;

    // Pass through position
    gl_Position = vec4(a_position, 0.0, 1.0);
}
