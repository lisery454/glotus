use std::collections::HashMap;

use glotus::{mesh::Vertex, transform::Transform};

fn main() {
    let mut app = glotus::App::new();
    app.init_window(1400, 960);

    app.create_shader_from_file(
        "shader_test",
        concat!(env!("CARGO_PKG_NAME"), "/assets/shaders/vs_0.vert"),
        concat!(env!("CARGO_PKG_NAME"), "/assets/shaders/fs_0.frag"),
    );

    app.create_material(
        "material_test",
        "shader_test",
        HashMap::new(),
        HashMap::new(),
    );
    app.create_mesh_from_data(
        "mesh_test",
        vec![
            Vertex::from_position(1.0, 1.0, -5.0),
            Vertex::from_position(1.0, -1.0, -5.0),
            Vertex::from_position(-1.0, -1.0, -5.0),
            Vertex::from_position(-1.0, 1.0, -5.0),
        ],
        vec![0, 1, 3, 1, 2, 3],
    );
    app.create_entity(
        "entity_test",
        Transform::default(),
        "material_test",
        "mesh_test",
    );
    app.run();
}
