use hecs::{Entity, World};
use raidillon_core::{EventHandler, GameEvent, InputAction, System, AssetManager, Model, Material, EventBus};
use raidillon_render::Camera;
use crate::FPSCameraController;
use glam::Vec3;

pub struct CameraSystem {
    controller: FPSCameraController,
    camera_entity: Entity,
    yaw: f32,
    pitch: f32,
}

impl CameraSystem {
    pub fn new(camera_entity: Entity) -> Self {
        Self {
            controller: FPSCameraController::new(Vec3::new(0.0, 0.0, 2.0)),
            camera_entity,
            yaw: -90.0,
            pitch: 0.0,
        }
    }

    pub fn update(&mut self, world: &mut World, dt: f32, mouse_delta: (f64, f64)) {
        // Apply mouse movement if there's any
        if mouse_delta.0 != 0.0 || mouse_delta.1 != 0.0 {
            self.yaw += mouse_delta.0 as f32 * self.controller.sensitivity;
            self.pitch -= mouse_delta.1 as f32 * self.controller.sensitivity;
            self.pitch = self.pitch.clamp(-89.0, 89.0);

            // Update front vector based on new yaw/pitch
            let yaw_rad = self.yaw.to_radians();
            let pitch_rad = self.pitch.to_radians();
            let front = Vec3::new(
                yaw_rad.cos() * pitch_rad.cos(),
                pitch_rad.sin(),
                yaw_rad.sin() * pitch_rad.cos(),
            ).normalize();
        }

        // Update camera component in the world
        if let Ok(cam) = world.query_one_mut::<&mut Camera>(self.camera_entity) {
            cam.eye = self.controller.position;
            cam.center = self.controller.position + self.controller.front();
        }
    }

    pub fn handle_input_action(&mut self, action: InputAction, dt: f32) {
        let front = self.controller.front();
        let right = front.cross(Vec3::Y).normalize();
        let frame_speed = self.controller.speed * dt;

        match action {
            InputAction::MoveForward => {
                self.controller.position += front * frame_speed;
            }
            InputAction::MoveBackward => {
                self.controller.position -= front * frame_speed;
            }
            InputAction::MoveLeft => {
                self.controller.position -= right * frame_speed;
            }
            InputAction::MoveRight => {
                self.controller.position += right * frame_speed;
            }
        }
    }

    pub fn resize_camera(&mut self, world: &mut World, width: u32, height: u32) {
        if let Ok(cam) = world.query_one_mut::<&mut Camera>(self.camera_entity) {
            cam.aspect = width as f32 / height as f32;
        }
    }
}

impl EventHandler for CameraSystem {
    fn handle(&mut self, event: &GameEvent) {
        match event {
            GameEvent::InputAction(_action) => {
                // Movement will be handled separately with delta time
                // This is just for event registration
            }
            GameEvent::WindowResize { width: _, height: _ } => {
                // Window resize will be handled separately with world access
            }
            _ => {}
        }
    }
}

impl System for CameraSystem {
    fn update(&mut self, world: &mut World, _resources: &AssetManager<dyn Model, dyn Material>, _events: &mut EventBus, _dt: f32) {
        // Camera update logic is handled separately with mouse input
        // This system mainly responds to events
    }

    fn handle_event(&mut self, event: &GameEvent, world: &mut World) {
        match event {
            GameEvent::WindowResize { width, height } => {
                self.resize_camera(world, *width, *height);
            }
            _ => {}
        }
    }

    fn name(&self) -> &'static str {
        "CameraSystem"
    }
} 