use cgmath::{Vector2, Vector3};

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex_coord: Vector2<f32>,
}

impl Vertex {
    pub fn from_position(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Vector3::<f32>::new(x, y, z),
            normal: Vector3::<f32>::new(0.0, 0.0, 0.0),
            tex_coord: Vector2::<f32>::new(0.0, 0.0),
        }
    }

    pub fn from_position_and_tex_coords(
        x: f32,
        y: f32,
        z: f32,
        tex_coord_x: f32,
        tex_coord_y: f32,
    ) -> Self {
        Self {
            position: Vector3::<f32>::new(x, y, z),
            normal: Vector3::<f32>::new(0.0, 0.0, 0.0),
            tex_coord: Vector2::<f32>::new(tex_coord_x, tex_coord_y),
        }
    }
}
