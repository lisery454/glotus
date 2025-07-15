use cgmath::{Vector2, Vector3};

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex_coords: Vector2<f32>,
}

impl Vertex {
    pub fn from_position(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Vector3::<f32>::new(x, y, z),
            normal: Vector3::<f32>::new(0.0, 0.0, 0.0),
            tex_coords: Vector2::<f32>::new(0.0, 0.0),
        }
    }
}
