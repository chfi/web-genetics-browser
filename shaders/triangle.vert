#version 450

layout (location = 0) in vec2 position;

out gl_PerVertex {
  vec4 gl_Position;
  // float gl_PointSize;
};


void main() {
  // float x = gl_Vertex

  // float x = float(int(gl_VertexIndex) - 1) * 0.8;
  // float y = float(int(gl_VertexIndex & 1) * 2 - 1) * 0.8;

  // vec2 pos = vec2((gl_VertexIndex << 1) & 2, gl_VertexIndex & 2);

  // vec4 pos = vec4(

  // gl_PointSize = 0.0;
	// gl_Position = vec4(x, y, 0.0, 1.0f);
  gl_Position = vec4(position, 0.0, 1.0f);
}
