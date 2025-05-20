#version 460

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 tex_coord;

layout(set = 0, binding = 0) uniform Data {
    mat4 world;
    mat4 view;
    mat4 proj;
} uniforms;

layout(location = 0) out vec2 frag_tex_coords;
layout(location = 1) out vec3 frag_normal;

void main() {
    gl_Position = uniforms.proj * uniforms.view * uniforms.world * vec4(position, 1.0);
    frag_tex_coords = tex_coord;
    frag_normal = mat3(uniforms.world) * normal;
}