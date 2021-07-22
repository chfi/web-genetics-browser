#version 450

layout (location = 0) out vec4 f_color;

layout (location = 0) in vec3 barycentric;

vec4 border_color = vec4(0.2, 0.2, 0.9, 1.0);
vec4 center_color = vec4(0.1, 0.1, 0.4, 1.0);

void main() {

  float dist = distance(barycentric, vec3(0.5));

  if (dist < 0.43) {
    float color_step = smoothstep(0.38, 0.42, dist);
    f_color = mix(center_color, border_color, color_step);
  } else {
    float color_step = smoothstep(0.43, 0.52, dist);
    float alpha = mix(1.0, 0.0, color_step);
    f_color = vec4(border_color.rgb, alpha);
  }
}
