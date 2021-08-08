#version 450


layout (set = 1, binding = 0) uniform texture2D u_texture;
layout (set = 1, binding = 1) uniform sampler u_sampler;

// layout (set = 0, binding = 1) uniform sampler2D u_sampler;
// layout (set = 0, binding = 0) uniform image2D u_texture;
// layout (set = 0, binding = 1) uniform sampler2D u_sampler;
// layout (set = 0, binding = 1) uniform sampler2D u_sampler;

layout (location = 0) in vec2 vs_uv;
layout (location = 1) in vec4 vs_color;

layout (location = 0) out vec4 f_color;

void main() {
  vec2 uv = vec2(vs_uv.x, vs_uv.y);
  // vec4 color = textureSampler
  // vec4 color = texture(u_texture, uv);
  // vec4 samplerrrrr = texture(sampler2D(u_texture, u_sampler), vec2(0.0, 0.0));
  vec4 color = texture(sampler2D(u_texture, u_sampler), uv);
  // vec4 color = texture(sampler2D(u_texture, u_sampler), uv);
  // vec4 color = vec4(1.0);

  vec4 tex_color = vec4(1.0, 1.0, 1.0, color.r);
  // vec4 tex_color = vec4(color.r, color.r, color.r, 1.0);
  f_color = vs_color * tex_color;
}
