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
            "assets/case_03_phong/shaders/vs.vert",
            "assets/case_03_phong/shaders/fs.frag",
        )
        .create_texture(
            "texture_brick",
            "assets/case_03_phong/textures/brick.png",
            WrappingMode::Repeat,
            WrappingMode::Repeat,
            FilteringMode::LinearMipmapLinear,
            FilteringMode::Linear,
        )
        .create_material(
            "material_test",
            "shader_test",
            [
                ("texture1".to_string(), UniformValue::Texture(0)),
                ("ambient_factor".to_string(), UniformValue::Float(0.2)),
                ("diffuse_factor".to_string(), UniformValue::Float(1.0)),
                ("specular_factor".to_string(), UniformValue::Float(0.5)),
                ("specular_shininess".to_string(), UniformValue::Float(256.0)),
            ]
            .into_iter()
            .collect(),
            [("texture_brick".to_string(), 0u32)].into_iter().collect(),
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
        .set_light_color(1.0, 0.5, 0.5, 1.0)
        .set_light_transform(Transform::from_position(10.0, 8.0, 6.0))
        .run();
}
