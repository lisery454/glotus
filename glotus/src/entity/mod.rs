use crate::{material::Material, transform::Transform};

pub struct Entity {
    pub transform: Transform,
    pub material_name: String,
    pub mesh_name: String,
}

impl Entity {
    pub fn new(transform: Transform, material_name: &str, mesh_name: &str) -> Self {
        Self {
            transform,
            material_name: material_name.to_string(),
            mesh_name: mesh_name.to_string(),
        }
    }
}
