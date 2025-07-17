pub mod projection_type;

use crate::transform::Transform;
use cgmath::{Deg, Matrix4, Ortho, PerspectiveFov, Rad, Vector3};
use projection_type::ProjectionType;

pub struct Camera {
    transform: Transform,
    fov: Deg<f32>,
    aspect_ratio: f32,
    near_plane: f32,
    far_plane: f32,
    projection_type: ProjectionType,
}

impl Camera {
    pub fn new(transform: Transform) -> Self {
        Self {
            transform,
            fov: Deg(45.0),
            aspect_ratio: 16.0 / 9.0,
            near_plane: 0.1,
            far_plane: 100.0,
            projection_type: ProjectionType::Perspective,
        }
    }

    pub fn get_transform(&self) -> &Transform {
        &self.transform
    }

    pub fn set_aspect_ratio(&mut self, width: u32, height: u32) {
        self.aspect_ratio = width as f32 / height as f32;
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_to_rh(
            self.transform.get_position(),
            self.get_forward(),
            self.get_up(),
        )
    }

    pub fn get_projection_matrix(&self) -> Matrix4<f32> {
        match self.projection_type {
            ProjectionType::Perspective => PerspectiveFov {
                fovy: Rad::from(self.fov),
                aspect: self.aspect_ratio,
                near: self.near_plane,
                far: self.far_plane,
            }
            .into(),
            ProjectionType::Orthographic => {
                let half_height = self.fov.0 / 2.0;
                let half_width = half_height * self.aspect_ratio;
                Ortho {
                    left: -half_width,
                    right: half_width,
                    bottom: -half_height,
                    top: half_height,
                    near: self.near_plane,
                    far: self.far_plane,
                }
                .into()
            }
        }
    }

    pub fn get_forward(&self) -> Vector3<f32> {
        self.transform.get_rotation() * -Vector3::unit_z()
    }

    pub fn get_right(&self) -> Vector3<f32> {
        self.transform.get_rotation() * Vector3::unit_x()
    }

    pub fn get_up(&self) -> Vector3<f32> {
        self.transform.get_rotation() * Vector3::unit_y()
    }
}
