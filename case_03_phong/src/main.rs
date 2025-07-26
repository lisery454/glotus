use glotus::{
    material::UniformValue,
    mesh::vertex::Vertex,
    texture::{FilteringMode, WrappingMode},
    transform::Transform,
};

fn main() {
    glotus::AppBuilder::new()
        .init_window(1440, 960)
        .create_shader_from_file(
            "shader_test",
            concat!(env!("CARGO_PKG_NAME"), "/assets/shaders/vs.vert"),
            concat!(env!("CARGO_PKG_NAME"), "/assets/shaders/fs.frag"),
        )
        .create_texture(
            "texture_diffuse",
            concat!(
                env!("CARGO_PKG_NAME"),
                "/assets/textures/texture_diffuse.png"
            ),
            WrappingMode::Repeat,
            WrappingMode::Repeat,
            FilteringMode::LinearMipmapLinear,
            FilteringMode::Linear,
        )
        .create_texture(
            "texture_specular",
            concat!(
                env!("CARGO_PKG_NAME"),
                "/assets/textures/texture_specular.png"
            ),
            WrappingMode::Repeat,
            WrappingMode::Repeat,
            FilteringMode::LinearMipmapLinear,
            FilteringMode::Linear,
        )
        .create_material(
            "material_test",
            "shader_test",
            [
                (
                    "material.diffuse_texture".to_string(),
                    UniformValue::get_texture(0),
                ),
                (
                    "material.specular_texture".to_string(),
                    UniformValue::get_texture(1),
                ),
                (
                    "material.ambient_factor".to_string(),
                    UniformValue::get_vector3_f32(0.2, 0.2, 0.2),
                ),
                (
                    "material.diffuse_factor".to_string(),
                    UniformValue::get_vector3_f32(1.0, 1.0, 1.0),
                ),
                (
                    "material.specular_factor".to_string(),
                    UniformValue::get_vector3_f32(0.8, 0.8, 0.8),
                ),
                (
                    "material.specular_shininess".to_string(),
                    UniformValue::get_float(256.0),
                ),
            ]
            .into_iter()
            .collect(),
            [
                ("texture_diffuse".to_string(), 0u32),
                ("texture_specular".to_string(), 1u32),
            ]
            .into_iter()
            .collect(),
        )
        .create_mesh_from_data(
            "mesh_test",
            vec![
                // back
                Vertex::from_position_and_normal_and_tex_coords(
                    -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 0.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 0.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 1.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 1.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    -0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 1.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 0.0,
                ),
                // front
                Vertex::from_position_and_normal_and_tex_coords(
                    -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 0.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 1.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 1.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    -0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 1.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0,
                ),
                // left
                Vertex::from_position_and_normal_and_tex_coords(
                    -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, 1.0, 0.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    -0.5, 0.5, -0.5, -1.0, 0.0, 0.0, 1.0, 1.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, 0.0, 1.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, 0.0, 1.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    -0.5, -0.5, 0.5, -1.0, 0.0, 0.0, 0.0, 0.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, 1.0, 0.0,
                ),
                // right
                Vertex::from_position_and_normal_and_tex_coords(
                    0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    0.5, 0.5, -0.5, 1.0, 0.0, 0.0, 1.0, 1.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 1.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 1.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    0.5, -0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 0.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0,
                ),
                // down
                Vertex::from_position_and_normal_and_tex_coords(
                    -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 0.0, 1.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 1.0, 1.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 1.0, 0.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 1.0, 0.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    -0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 0.0, 0.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 0.0, 1.0,
                ),
                // up
                Vertex::from_position_and_normal_and_tex_coords(
                    -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 1.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 1.0, 1.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    -0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 0.0,
                ),
                Vertex::from_position_and_normal_and_tex_coords(
                    -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 1.0,
                ),
            ],
            vec![],
        )
        .create_entity(
            "entity_test",
            Transform::new(),
            "material_test",
            "mesh_test",
        )
        .set_camera_transform(Transform::from_position(0.0, 0.0, 10.0))
        .set_light_color(1.0, 1.0, 1.0, 1.0)
        .set_light_transform(Transform::from_position(10.0, 8.0, 6.0))
        .run();
}
