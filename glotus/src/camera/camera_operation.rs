use cgmath::{Deg, InnerSpace, Quaternion, Rad, Rotation3, Vector3};

use super::Camera;

pub enum CameraMovement {
    Forward { velocity: f32 },
    Backward { velocity: f32 },
    Left { velocity: f32 },
    Right { velocity: f32 },
    Up { velocity: f32 },
    Down { velocity: f32 },
}

impl Camera {
    pub fn process_move(&mut self, movement: CameraMovement, delta_time: f32) {
        match movement {
            CameraMovement::Forward { velocity } => {
                self.transform.position += self.get_forward() * velocity * delta_time;
            }
            CameraMovement::Backward { velocity } => {
                self.transform.position -= self.get_forward() * velocity * delta_time;
            }
            CameraMovement::Right { velocity } => {
                self.transform.position += self.get_right() * velocity * delta_time;
            }
            CameraMovement::Left { velocity } => {
                self.transform.position -= self.get_right() * velocity * delta_time;
            }
            CameraMovement::Up { velocity } => {
                self.transform.position += self.get_up() * velocity * delta_time;
            }
            CameraMovement::Down { velocity } => {
                self.transform.position -= self.get_up() * velocity * delta_time;
            }
        }
    }

    pub fn process_turn(
        &mut self,
        xoffset: f32,
        yoffset: f32,
        sensitivity: f32,
        constrain_pitch: bool,
    ) {
        // 将偏移量转换为弧度
        let yaw = Rad(-xoffset * sensitivity);
        let pitch = Rad(-yoffset * sensitivity);

        // 创建绕Y轴(偏航)和X轴(俯仰)的旋转四元数
        let yaw_rotation = Quaternion::from_axis_angle(Vector3::unit_y(), yaw);
        let pitch_rotation = Quaternion::from_axis_angle(Vector3::unit_x(), pitch);

        // 应用偏航旋转
        self.transform.rotation = yaw_rotation * self.transform.rotation;

        // 应用俯仰旋转前检查角度限制
        let new_rotation = self.transform.rotation * pitch_rotation;

        // 从新旋转中提取前向向量
        let forward = new_rotation * Vector3::unit_z();

        // 计算当前俯仰角(与水平面的夹角)
        let pitch_angle = forward.y.asin(); // 返回弧度值

        if constrain_pitch {
            // 限制俯仰角度在±89度内
            if pitch_angle.abs() < 89.0f32.to_radians() {
                self.transform.rotation = new_rotation;
            }
        }

        // 确保四元数归一化
        self.transform.rotation = self.transform.rotation.normalize();
    }

    pub fn process_zoom(&mut self, yoffset: f32, sensitivity: f32) {
        // 计算新的FOV值
        let mut new_fov = self.fov.0 - yoffset * sensitivity;

        // 限制FOV范围（通常在1.0到120度之间）
        new_fov = new_fov.clamp(1.0, 120.0);

        self.fov = Deg(new_fov);
    }
}
