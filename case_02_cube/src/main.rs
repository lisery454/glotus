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
            "texture_brick",
            concat!(env!("CARGO_PKG_NAME"), "/assets/textures/brick.png"),
            WrappingMode::Repeat,
            WrappingMode::Repeat,
            FilteringMode::LinearMipmapLinear,
            FilteringMode::Linear,
        )
        .create_material(
            "material_test",
            "shader_test",
            [("texture1".to_string(), UniformValue::Texture(0))]
                .into_iter()
                .collect(),
            [("texture_brick".to_string(), 0u32)].into_iter().collect(),
        )
        .create_mesh_from_data(
            "mesh_test",
            vec![
                // back
                Vertex::from_position_and_tex_coords(-0.5, -0.5, -0.5, 0.0, 0.0),
                Vertex::from_position_and_tex_coords(0.5, -0.5, -0.5, 1.0, 0.0),
                Vertex::from_position_and_tex_coords(0.5, 0.5, -0.5, 1.0, 1.0),
                Vertex::from_position_and_tex_coords(0.5, 0.5, -0.5, 1.0, 1.0),
                Vertex::from_position_and_tex_coords(-0.5, 0.5, -0.5, 0.0, 1.0),
                Vertex::from_position_and_tex_coords(-0.5, -0.5, -0.5, 0.0, 0.0),
                // front
                Vertex::from_position_and_tex_coords(-0.5, -0.5, 0.5, 0.0, 0.0),
                Vertex::from_position_and_tex_coords(0.5, -0.5, 0.5, 1.0, 0.0),
                Vertex::from_position_and_tex_coords(0.5, 0.5, 0.5, 1.0, 1.0),
                Vertex::from_position_and_tex_coords(0.5, 0.5, 0.5, 1.0, 1.0),
                Vertex::from_position_and_tex_coords(-0.5, 0.5, 0.5, 0.0, 1.0),
                Vertex::from_position_and_tex_coords(-0.5, -0.5, 0.5, 0.0, 0.0),
                // left
                Vertex::from_position_and_tex_coords(-0.5, 0.5, 0.5, 1.0, 0.0),
                Vertex::from_position_and_tex_coords(-0.5, 0.5, -0.5, 1.0, 1.0),
                Vertex::from_position_and_tex_coords(-0.5, -0.5, -0.5, 0.0, 1.0),
                Vertex::from_position_and_tex_coords(-0.5, -0.5, -0.5, 0.0, 1.0),
                Vertex::from_position_and_tex_coords(-0.5, -0.5, 0.5, 0.0, 0.0),
                Vertex::from_position_and_tex_coords(-0.5, 0.5, 0.5, 1.0, 0.0),
                // right
                Vertex::from_position_and_tex_coords(0.5, 0.5, 0.5, 1.0, 0.0),
                Vertex::from_position_and_tex_coords(0.5, 0.5, -0.5, 1.0, 1.0),
                Vertex::from_position_and_tex_coords(0.5, -0.5, -0.5, 0.0, 1.0),
                Vertex::from_position_and_tex_coords(0.5, -0.5, -0.5, 0.0, 1.0),
                Vertex::from_position_and_tex_coords(0.5, -0.5, 0.5, 0.0, 0.0),
                Vertex::from_position_and_tex_coords(0.5, 0.5, 0.5, 1.0, 0.0),
                // down
                Vertex::from_position_and_tex_coords(-0.5, -0.5, -0.5, 0.0, 1.0),
                Vertex::from_position_and_tex_coords(0.5, -0.5, -0.5, 1.0, 1.0),
                Vertex::from_position_and_tex_coords(0.5, -0.5, 0.5, 1.0, 0.0),
                Vertex::from_position_and_tex_coords(0.5, -0.5, 0.5, 1.0, 0.0),
                Vertex::from_position_and_tex_coords(-0.5, -0.5, 0.5, 0.0, 0.0),
                Vertex::from_position_and_tex_coords(-0.5, -0.5, -0.5, 0.0, 1.0),
                // up
                Vertex::from_position_and_tex_coords(-0.5, 0.5, -0.5, 0.0, 1.0),
                Vertex::from_position_and_tex_coords(0.5, 0.5, -0.5, 1.0, 1.0),
                Vertex::from_position_and_tex_coords(0.5, 0.5, 0.5, 1.0, 0.0),
                Vertex::from_position_and_tex_coords(0.5, 0.5, 0.5, 1.0, 0.0),
                Vertex::from_position_and_tex_coords(-0.5, 0.5, 0.5, 0.0, 0.0),
                Vertex::from_position_and_tex_coords(-0.5, 0.5, -0.5, 0.0, 1.0),
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
        .run();
}
