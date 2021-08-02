#version 450

layout (location = 0) in vec2 position;

layout (location = 0) out vec3 barycentric;

layout (set = 0, binding = 0) uniform UBO {
  mat4 view_transform;
} ubo;

out gl_PerVertex {
  vec4 gl_Position;
};

float neg_log_10(in float p) {
  float log10 = log(10.0);
  float p_log = log(p) / log10;

  float neg_p_log = -p_log;

  float max_y = 10.0;

  return neg_p_log / max_y;
}


void main() {
  float y = neg_log_10(position.y);
  vec4 pos = ubo.view_transform * vec4(position.x, y, 0.0, 1.0);

  float b_x;
  float b_y;
  float b_z;

  float del = 0.035;

  if ((gl_VertexIndex % 3) == 0) {
    barycentric = vec3(1.0, 0.0, 0.0);
    pos.y += del;
  } else if ((gl_VertexIndex % 3) == 1) {
    barycentric = vec3(0.0, 1.0, 0.0);
    pos.x -= (del * 0.717);
    pos.y -= (del * 0.717);
  } else {
    barycentric = vec3(0.0, 0.0, 1.0);
    pos.x += (del * 0.717);
    pos.y -= (del * 0.717);
  }

  gl_Position = vec4(pos.xy, 0.0, 1.0f);
}
