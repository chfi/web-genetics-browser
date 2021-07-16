#version 450

layout (location = 0) out vec4 f_color;

layout (location = 0) in vec3 barycentric;

void main() {

  vec4 color = vec3(0.0);

  if (barycentric.x > 0.4 && barycentric.x < 0.6) {
    color.r = barycentric.x;
    color.a = 1.0;
  }

  if (barycentric.y > 0.4 && barycentric.y < 0.6) {
    color.g = barycentric.y;
    color.a = 1.0;
  }

  if (barycentric.z > 0.4 && barycentric.z < 0.6) {
    color.b = barycentric.z;
    color.a = 1.0;
  }

  f_color = color;

  /*
  if (distance(barycentric, vec3(0.5)) < 0.2) {
  // if (length(barycentric) > 0.3) {
  // if (barycentric.y < 0.5) {
    f_color = vec4(barycentric, 1.0);
    // f_color =
    // f_color = vec4(1.0, 0.0, 0.0, 1.0);
  } else {
    f_color = vec4(1.0);
  }
  // f_color = vec4(1.0);
  */
}
