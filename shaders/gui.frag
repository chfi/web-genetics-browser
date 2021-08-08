#version 450

layout (set = 1, binding = 0) uniform texture2D u_texture;
layout (set = 1, binding = 1) uniform sampler u_sampler;

layout (location = 0) in vec2 vs_uv;
layout (location = 1) in vec4 vs_color;

layout (location = 0) out vec4 f_color;

void main() {
  vec2 uv = vec2(vs_uv.x, vs_uv.y);

  vec4 color = texture(sampler2D(u_texture, u_sampler), uv);

  f_color = vs_color * color;
}
