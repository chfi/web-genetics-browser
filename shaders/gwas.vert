#version 450

layout (location = 0) in vec2 position;

layout (location = 0) out vec3 barycentric;

layout (set = 0, binding = 0) uniform UBO {
  // float scale;
  // float s
  vec4 scale;
} ubo;

out gl_PerVertex {
  vec4 gl_Position;
  // float gl_PointSize;
};


void main() {
  // vec4 pos = ubo.view * ubo.projection * vec4(position, 0.0, 1.0);

  // float angle = 1.0;

  float scale = ubo.scale.x;
  // float scale = 0.0;

  mat2 rotate = mat2(cos(scale), -sin(scale),
                     sin(scale),  cos(scale));

  // vec2 rot_pos = rotate * position;


  vec2 rot_pos = rotate * position;
  // vec2 pos = position;
  // pos.x += scale * 0.01;
  gl_Position = vec4(rot_pos, 0.0, 1.0f);

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
