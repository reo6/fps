use glam::{Mat4, Quat, Vec3};

#[derive(Copy, Clone)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation:    Quat,
    pub scale:       Vec3,
}

impl Transform {
    pub fn matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ModelHandle(pub usize);
