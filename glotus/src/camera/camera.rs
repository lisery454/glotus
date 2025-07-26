use cgmath::{Deg, Matrix4, Ortho, PerspectiveFov, Rad, Vector3};

use crate::transform::Transform;

use super::projection_type::ProjectionType;

pub struct Camera {
    pub(crate) transform: Transform,
    pub(crate) fov: Deg<f32>,
    pub(crate) aspect_ratio: f32,
    pub(crate) near_plane: f32,
    pub(crate) far_plane: f32,
    pub(crate) projection_type: ProjectionType,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            transform: Transform::default(),
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

    pub fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform
    }

    pub fn set_aspect_ratio(&mut self, width: u32, height: u32) {
        self.aspect_ratio = width as f32 / height as f32;
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    pub(crate) fn get_view_matrix(&self) -> [[f32; 4]; 4] {
        Matrix4::look_to_rh(
            self.transform.get_position().get_data(),
            self.get_forward(),
            self.get_up(),
        ).into()
    }

    pub(crate) fn get_projection_matrix(&self) -> [[f32; 4]; 4] {
       let matrix :Matrix4<f32>= match self.projection_type {
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
        };

        matrix.into()
    }

    pub fn get_forward(&self) -> Vector3<f32> {
        self.transform.get_rotation().get_data() * -Vector3::unit_z()
    }

    pub fn get_right(&self) -> Vector3<f32> {
        self.transform.get_rotation().get_data() * Vector3::unit_x()
    }

    pub fn get_up(&self) -> Vector3<f32> {
        self.transform.get_rotation().get_data() * Vector3::unit_y()
    }
}
