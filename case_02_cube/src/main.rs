use glotus::{
    material::UniformValue,
    mesh::Vertex,
    texture::{FilteringMode, WrappingMode},
    transform::Transform,
};

fn main() {
    let mut app = glotus::App::new();
    app.init_window(1440, 960);
    app.create_shader_from_file(
        "shader_test",
        concat!(env!("CARGO_PKG_NAME"), "/assets/shaders/vs.vert"),
        concat!(env!("CARGO_PKG_NAME"), "/assets/shaders/fs.frag"),
    );
    app.create_texture(
        "texture_brick",
        concat!(env!("CARGO_PKG_NAME"), "/assets/textures/brick.png"),
        WrappingMode::Repeat,
        WrappingMode::Repeat,
        FilteringMode::LinearMipmapLinear,
        FilteringMode::Linear,
    );
    app.create_material(
        "material_test",
        "shader_test",
        [("texture1".to_string(), UniformValue::Texture(0))]
            .into_iter()
            .collect(),
        [("texture_brick".to_string(), 0u32)].into_iter().collect(),
    );
    app.create_mesh_from_data(
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
    );
    app.create_entity(
        "entity_test",
        Transform::default(),
        "material_test",
        "mesh_test",
    );
    app.set_camera_transform(Transform::from_position(0.0, 0.0, 10.0));
    app.run();
}
