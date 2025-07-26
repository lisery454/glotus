#version 460 core
#include "glotus.glsl"
layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec3 texcoord;

out vec2 TexCoord;

void main() {
  gl_Position =
      projection_matrix * view_matrix * model_matrix * vec4(position, 1.0f);
  TexCoord = vec2(texcoord.x, texcoord.y);
}