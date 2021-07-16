#version 450

out gl_PerVertex {
  vec4 gl_Position;
};


void main() {
  // float x = gl_Vertex

  float x = float(int(gl_VertexIndex) - 1);
  float y = float(int(gl_VertexIndex & 1) * 2 - 1);

  // vec2 pos = vec2((gl_VertexIndex << 1) & 2, gl_VertexIndex & 2);

	gl_Position = vec4(x, y, 0.0, 1.0f);
}
