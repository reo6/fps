use glam::{Mat4, Vec3};

#[derive(Copy, Clone)]
pub struct Camera {
    pub eye:    Vec3,
    pub center: Vec3,
    pub up:     Vec3,
    pub fovy:   f32,
    pub aspect: f32,
    pub znear:  f32,
    pub zfar:   f32,
}

impl Camera {
    pub fn view(&self) -> Mat4 {
        Mat4::look_at_rh(self.eye, self.center, self.up)
    }
    pub fn projection(&self) -> Mat4 {
        Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar)
    }
    pub fn view_proj(&self) -> Mat4 {
        self.projection() * self.view()
    }
}
