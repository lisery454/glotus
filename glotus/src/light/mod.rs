use cgmath::Vector4;

use crate::transform::{self, Transform};

#[derive(Debug)]
pub struct Light {
    transform: Transform,
    color: Vector4<f32>,
}

impl Light {
    pub fn new() -> Self {
        Self {
            transform: Transform::new(),
            color: Vector4 {
                x: 1f32,
                y: 1f32,
                z: 1f32,
                w: 1f32,
            },
        }
    }

    pub fn get_transform(&self) -> &Transform {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    pub fn get_color(&self) -> Vector4<f32> {
        self.color
    }

    pub fn set_color(&mut self, color: Vector4<f32>) {
        self.color = color;
    }
}
