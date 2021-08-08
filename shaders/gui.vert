#version 450

layout (location = 0) in vec2 pos;
layout (location = 1) in vec2 uv;
layout (location = 2) in vec4 color;
// layout (location = 2) in uint i_color;

layout (location = 0) out vec2 vs_uv;
layout (location = 1) out vec4 vs_color;

layout (set = 0, binding = 0) uniform UBO {
  vec4 screen_dims;
} ubo;

// taken from the egui glium example
    // 0-1 linear  from  0-255 sRGB
vec3 linear_from_srgb(vec3 srgb) {
  bvec3 cutoff = lessThan(srgb, vec3(10.31475));
  vec3 lower = srgb / vec3(3294.6);
  vec3 higher = pow((srgb + vec3(14.025)) / vec3(269.025), vec3(2.4));
  return mix(higher, lower, cutoff);
}

vec4 linear_from_srgba(vec4 srgba) {
  vec3 srgb = srgba.xyz * 255.0;
  return vec4(linear_from_srgb(srgb), srgba.a);
}

void main() {
  gl_Position = vec4(
                     2.0 * pos.x / ubo.screen_dims.x - 1.0,
                     1.0 - 2.0 * pos.y / ubo.screen_dims.y,
                     0.0,
                     1.0
                     );

  vs_color = color;
  vs_uv = uv;
}
