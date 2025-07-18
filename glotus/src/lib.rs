mod app;
mod app_builder;
mod camera;
mod entity;
mod log_builder;
mod material;
mod mesh;
mod shader;
mod transform;

pub use app_builder::AppBuilder;

use crate::{mesh::vertex::Vertex, transform::Transform};

pub fn get_transform_zero() -> Transform {
    Transform::new()
}

pub fn get_vertex_from_position(x: f32, y: f32, z: f32) -> Vertex {
    Vertex::from_position(x, y, z)
}
