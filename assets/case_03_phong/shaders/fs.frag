#version 460 core
out vec4 frag_color;

in vec3 normal_in_world;
in vec3 frag_pos_in_world;
in vec2 tex_coord;

struct Material {
  vec3 ambient_factor;        // 材质环境光反射系数
  vec3 diffuse_factor;        // 材质漫反射系数
  vec3 specular_factor;       // 材质镜面反射系数
  float specular_shininess;   // 材质高光指数（反光度）
  sampler2D diffuse_texture;  // 漫反射贴图
  sampler2D specular_texture; // 高光贴图
};

uniform Material material;

uniform vec3 light_position;
uniform vec3 view_position;
uniform vec4 light_color;

void main() {
  vec3 normal = normalize(normal_in_world);
  vec3 light_dir = normalize(light_position - frag_pos_in_world);
  vec3 view_dir = normalize(view_position - frag_pos_in_world);
  vec3 reflect_dir = reflect(-light_dir, normal);

  vec3 ambient = material.ambient_factor *
                 texture(material.diffuse_texture, tex_coord).xyz;
  vec3 diffuse = material.diffuse_factor * max(dot(normal, light_dir), 0.0) *
                 texture(material.diffuse_texture, tex_coord).xyz;
  vec3 specular =
      material.specular_factor *
      pow(max(dot(view_dir, reflect_dir), 0.0), material.specular_shininess) *
      texture(material.specular_texture, tex_coord).xyz;

  // all
  vec3 result = (ambient + diffuse + specular) * light_color.xyz;
  frag_color = vec4(result, 1.0);
}