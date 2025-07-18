use glam::Vec3;
use std::hash::Hash;

use super::Input;

#[derive(Debug, Clone)]
pub struct FPSCameraController {
    pub position: Vec3,
    yaw:   f32,
    pitch: f32,
    pub speed: f32,
    pub sensitivity: f32,
}

impl FPSCameraController {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            yaw: -90.0,
            pitch: 0.0,
            speed: 3.0,
            sensitivity: 0.1,
        }
    }

    pub fn update<A>(&mut self,
                     input: &Input<A>,
                     dt: f32,
                     mouse_enabled: bool,
                     actions: (A, A, A, A))
    where
        A: Copy + Eq + Hash,
    {
        let (forward, backward, left, right) = actions;

        // Mouse look
        if mouse_enabled {
            let (dx, dy) = input.mouse_delta();
            self.yaw   += dx as f32 * self.sensitivity;
            self.pitch -= dy as f32 * self.sensitivity;
            self.pitch = self.pitch.clamp(-89.0, 89.0);
        }

        // Movement
        let front = self.front();
        let right_vec = front.cross(Vec3::Y).normalize();
        let frame_speed = self.speed * dt;

        if input.action_held(forward) {
            self.position += front * frame_speed;
        }
        if input.action_held(backward) {
            self.position -= front * frame_speed;
        }
        if input.action_held(left) {
            self.position -= right_vec * frame_speed;
        }
        if input.action_held(right) {
            self.position += right_vec * frame_speed;
        }
    }

    pub fn front(&self) -> Vec3 {
        let yaw_rad   = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();
        Vec3::new(
            yaw_rad.cos() * pitch_rad.cos(),
            pitch_rad.sin(),
            yaw_rad.sin() * pitch_rad.cos(),
        )
        .normalize()
    }
} 