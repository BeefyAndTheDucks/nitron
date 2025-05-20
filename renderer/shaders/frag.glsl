#version 450

layout(location = 0) in vec2 frag_tex_coords;
layout(location = 1) in vec3 frag_normal;

layout(location = 0) out vec4 f_color;

layout(set = 0, binding = 1) uniform sampler2D tex_sampler;

void main() {
    vec4 tex_color = texture(tex_sampler, frag_tex_coords);
    vec3 light_dir = normalize(vec3(1.0, 1.0, 1.0));
    float diff = max(dot(normalize(frag_normal), light_dir), 0.0);
    vec3 diffuse = diff * vec3(1.0);
    vec3 ambient = vec3(0.1);
    f_color = vec4((ambient + diffuse) * tex_color.rgb, tex_color.a);
}
