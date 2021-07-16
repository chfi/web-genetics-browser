#version 450

layout (location = 0) out vec4 f_color;

layout (location = 0) in vec3 barycentric;

void main() {

  f_color = vec4(barycentric, 1.0);

  if (distance(barycentric, vec3(0.5)) > 0.45) {
    f_color.a = 0.0;
  }
}
