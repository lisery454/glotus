use std::collections::HashMap;

use log::info;

use super::{mesh::Mesh, vertex::Vertex};

pub struct MeshManager {
    meshes: HashMap<String, Mesh>,
}

impl MeshManager {
    pub fn new() -> Self {
        Self {
            meshes: HashMap::new(),
        }
    }

    pub fn has(&self, mesh_name: &str) -> bool{
        self.meshes.contains_key(mesh_name)
    }

    pub fn get(&self , mesh_name: &str) -> Option<&Mesh> {
        self.meshes.get(mesh_name)
    }

    pub fn create_mesh_from_data(
        &mut self,
        mesh_name: &str,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    ) {
        let mesh = Mesh::new(vertices, indices);

        info!("success add mesh <{:?}>", mesh_name);
        self.meshes.insert(mesh_name.to_string(), mesh);
    }
}
