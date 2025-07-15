use cgmath::{EuclideanSpace, Matrix4, Point3, Quaternion, Vector3};

#[derive(Debug)]
pub struct Transform {
    pub position: Point3<f32>,
    pub rotation: Quaternion<f32>,
    pub scale: Vector3<f32>,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: Point3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn to_matrix(&self) -> Matrix4<f32> {
        // 1. 生成缩放矩阵
        let scale_matrix = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);

        // 2. 生成旋转矩阵（从四元数转换）
        let rotation_matrix = Matrix4::from(self.rotation);

        // 3. 生成平移矩阵
        let translation_matrix = Matrix4::from_translation(self.position.to_vec());

        // 4. 组合变换：T * R * S（注意顺序！）
        translation_matrix * rotation_matrix * scale_matrix
    }
}
