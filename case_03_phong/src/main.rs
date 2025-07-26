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
        "texture_diffuse",
        concat!(
            env!("CARGO_PKG_NAME"),
            "/assets/textures/texture_diffuse.png"
        ),
        WrappingMode::Repeat,
        WrappingMode::Repeat,
        FilteringMode::LinearMipmapLinear,
        FilteringMode::Linear,
    );
    app.create_texture(
        "texture_specular",
        concat!(
            env!("CARGO_PKG_NAME"),
            "/assets/textures/texture_specular.png"
        ),
        WrappingMode::Repeat,
        WrappingMode::Repeat,
        FilteringMode::LinearMipmapLinear,
        FilteringMode::Linear,
    );
    app.create_material(
        "material_test",
        "shader_test",
        [
            (
                "material.diffuse_texture".to_string(),
                UniformValue::Texture(0),
            ),
            (
                "material.specular_texture".to_string(),
                UniformValue::Texture(1),
            ),
            (
                "material.ambient_factor".to_string(),
                UniformValue::Vector3([0.2, 0.2, 0.2]),
            ),
            (
                "material.diffuse_factor".to_string(),
                UniformValue::Vector3([1.0, 1.0, 1.0]),
            ),
            (
                "material.specular_factor".to_string(),
                UniformValue::Vector3([0.8, 0.8, 0.8]),
            ),
            (
                "material.specular_shininess".to_string(),
                UniformValue::Float(256.0),
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
    );
    app.create_mesh_from_data(
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
            Vertex::from_position_and_normal_and_tex_coords(0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 1.0),
            Vertex::from_position_and_normal_and_tex_coords(0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 1.0),
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
            Vertex::from_position_and_normal_and_tex_coords(0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0),
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
            Vertex::from_position_and_normal_and_tex_coords(0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0),
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
            Vertex::from_position_and_normal_and_tex_coords(0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0),
            Vertex::from_position_and_normal_and_tex_coords(0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0),
            Vertex::from_position_and_normal_and_tex_coords(
                -0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 0.0,
            ),
            Vertex::from_position_and_normal_and_tex_coords(
                -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 1.0,
            ),
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
    app.set_light_color([1.0, 1.0, 1.0, 1.0]);
    app.set_light_transform(Transform::from_position(10.0, 8.0, 6.0));
    app.run();
}
