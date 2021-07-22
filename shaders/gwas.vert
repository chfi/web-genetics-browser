#version 450

layout (location = 0) in vec2 position;

layout (location = 0) out vec3 barycentric;

layout (set = 0, binding = 0) uniform UBO {
  mat4 view_transform;
  // float scale;
  // float s
  // vec4 scale;
} ubo;

out gl_PerVertex {
  vec4 gl_Position;
  // float gl_PointSize;
};


void main() {

  vec4 pos = ubo.view_transform * vec4(position, 0.0, 1.0);

  gl_Position = vec4(pos.xy, 0.0, 1.0f);

  float b_x;
  float b_y;
  float b_z;

  if ((gl_VertexIndex % 3) == 0) {
    barycentric = vec3(1.0, 0.0, 0.0);
  } else if ((gl_VertexIndex % 3) == 1) {
    barycentric = vec3(0.0, 1.0, 0.0);
  } else {
    barycentric = vec3(0.0, 0.0, 1.0);
  }
}
