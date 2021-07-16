#version 450

layout (location = 0) out vec4 f_color;

layout (location = 0) in vec3 barycentric;

void main() {


  /*
  vec3 color = vec3(0.0);


  // uint counter = 0;

  if (barycentric.x > 0.4 && barycentric.x < 0.6) {
    color.r = barycentric.x;
    // counter += 1;
  }

  if (barycentric.y > 0.4 && barycentric.y < 0.6) {
    color.g = barycentric.y;
    // counter += 1;
  }

  if (barycentric.z > 0.4 && barycentric.z < 0.6) {
    color.b = barycentric.z;
    // counter += 1;
  }

  if (color.r > 0.0 && color.g > 0.0 && color.b > 0.0) {
    f_color = vec4(color, 1.0);
  } else {
    f_color = vec4(0.0);
  }
  */

  // if (counter > 2) {
  //   color.a = 1.0;
  // }

  // f_color = vec4(color, 1.0);

  // f_color = vec4(length(barycentric / 3.0) / 2.0);
  // f_color = vec4(length(barycentric / 100.0));


  // if ((barycentric.x + barycentric.y + barycentric.z) > 1.3) {
  f_color = vec4(barycentric, 1.0);

  if (barycentric.x < 0.1
      || barycentric.y < 0.1
      || barycentric.z < 0.1) {
    f_color.a = 0.0;
  }
  // }
/*else {
    f_color = vec4(0.0);
  }*/

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
