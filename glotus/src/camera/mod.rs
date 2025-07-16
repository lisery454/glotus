use crate::transform::Transform;

pub struct Camera {
    pub transform: Transform,
}

impl Camera {
    pub fn new(transform: Transform) -> Self {
        Self { transform }
    }
}
