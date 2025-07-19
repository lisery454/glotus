use std::collections::HashMap;

fn main() {
    glotus::AppBuilder::new()
        .init_window(1440, 960)
        .create_shader_from_file("shader_test", "shaders/vs_0.vs", "shaders/fs_0.fs")
        .create_material("material_test", "shader_test", HashMap::new())
        .create_mesh_from_data(
            "mesh_test",
            vec![
                glotus::get_vertex_from_position(1.0, 1.0, -5.0),
                glotus::get_vertex_from_position(1.0, -1.0, -5.0),
                glotus::get_vertex_from_position(-1.0, -1.0, -5.0),
                glotus::get_vertex_from_position(-1.0, 1.0, -5.0),
            ],
            vec![0, 1, 3, 1, 2, 3],
        )
        .create_entity(
            "entity_test",
            glotus::get_transform_zero(),
            "material_test",
            "mesh_test",
        )
        .run();
}
