use glam::{Mat4, Quat, Vec3};
use hecs::World;

/// ------------ components ------------
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

#[derive(Clone)]
pub struct MeshHandle(pub usize);

/// ------------ systems ------------
pub fn rotation_system(world: &mut World, dt: f32) {
    for (_, transform) in world.query_mut::<&mut Transform>() {
        transform.rotation *= Quat::from_rotation_y(dt);
    }
}

/// Update the aspect ratio for all camera components in the world.
pub fn set_camera_aspect(world: &mut World, aspect: f32) {
    for (_, cam) in world.query_mut::<&mut crate::camera::Camera>() {
        cam.aspect = aspect;
    }
}
