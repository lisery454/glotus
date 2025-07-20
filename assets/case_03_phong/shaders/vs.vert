#version 460 core
layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec3 texcoord;

out vec3 frag_pos_in_world;
out vec3 normal_in_world;
out vec2 tex_coord;

uniform mat3 normal_matrix;
uniform mat4 model_matrix;
uniform mat4 view_matrix;
uniform mat4 projection_matrix;

void main() {
  // 点在世界坐标中的位置
  frag_pos_in_world = vec3(model_matrix * vec4(position, 1.0));
  // 法线在世界坐标中的向量
  normal_in_world = normal_matrix * normal;
  // 纹理坐标
  tex_coord = vec2(texcoord.x, texcoord.y);
  // 屏幕位置
  gl_Position = projection_matrix * view_matrix * vec4(frag_pos_in_world, 1.0);
}