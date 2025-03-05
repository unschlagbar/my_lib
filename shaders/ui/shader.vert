#version 450

layout(binding = 0) uniform UniformBufferObject {
    mat4 view;
    mat4 proj;
} ubo;

layout(location = 0) in vec4 inColor;
layout(location = 1) in vec4 inBorderColor;
layout(location = 2) in float inBorder;
layout(location = 3) in float x; // Instanz-Daten
layout(location = 4) in float y; // Instanz-Daten
layout(location = 5) in float width; // Instanz-Daten
layout(location = 6) in float height; // Instanz-Daten
layout(location = 7) in float inCorner; // Instanz-Daten
layout(location = 8) in uint inMode; // Instanz-Daten

layout(location = 0) out vec2 fragTexCoord;
layout(location = 1) out vec4 fragColor;
layout(location = 2) out vec4 fragBorderColor;
layout(location = 3) out float fragWidth; // Instanz-Daten
layout(location = 4) out float fragHeight; // Instanz-Daten
layout(location = 5) out float fragBorder;
layout(location = 6) out float fragCorner;
layout(location = 7) out uint fragMode; 

void main() {
    vec2 uv = vec2(((gl_VertexIndex << 1) & 2) >> 1, (gl_VertexIndex & 2) >> 1);
    gl_Position = ubo.proj * vec4(vec2(x, y) + vec2(width, height) * uv, 0.0, 1.0);
    fragTexCoord = uv;
    fragColor = inColor;
    fragBorderColor = inBorderColor;
    fragWidth = width;
    fragHeight = height;
    fragBorder = inBorder;
    fragCorner = inCorner;
    fragMode = inMode;
}
