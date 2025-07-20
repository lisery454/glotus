#version 460 core
out vec4 frag_color;

in vec3 normal_in_world;
in vec3 frag_pos_in_world;
in vec2 tex_coord;

uniform float ambient_factor;     // 材质环境光反射系数
uniform float diffuse_factor;     // 材质漫反射系数
uniform float specular_factor;    // 材质镜面反射系数
uniform float specular_shininess; // 材质高光指数（反光度）

uniform sampler2D texture1;
uniform vec3 light_position;
uniform vec3 view_position;
uniform vec4 light_color;

void main() {
  vec3 normal = normalize(normal_in_world);
  vec3 light_dir = normalize(light_position - frag_pos_in_world);
  vec3 view_dir = normalize(view_position - frag_pos_in_world);
  vec3 reflect_dir = reflect(-light_dir, normal);

  float ambient = ambient_factor;
  float diffuse = diffuse_factor * max(dot(normal, light_dir), 0.0);
  float specular = specular_factor * pow(max(dot(view_dir, reflect_dir), 0.0),
                                         specular_shininess);

  // all
  vec3 objectColor = texture(texture1, tex_coord).xyz;
  vec3 result = (ambient + diffuse + specular) * light_color.xyz * objectColor;
  frag_color = vec4(result, 1.0);
}