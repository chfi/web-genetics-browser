#version 450

layout (location = 0) out vec4 f_color;

layout (location = 0) in vec3 barycentric;

vec4 border_color = vec4(0.2, 0.2, 0.9, 1.0);
vec4 center_color = vec4(0.1, 0.1, 0.4, 1.0);

void main() {

  float dist = distance(barycentric, vec3(0.5));

  if (dist < 0.4) {
    f_color = center_color;
  } else if (dist < 0.48) {
    f_color = border_color;
  } else {
    f_color = vec4(0.0);
  }
}
