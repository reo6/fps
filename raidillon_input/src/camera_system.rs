use glam::Vec3;
use hecs::World;

use crate::camera::FPSCameraController;
use crate::Action;
use raidillon_core::{System, AssetManager, EventHandler, GameEvent};
use raidillon_render::Camera;

pub struct CameraSystem {
    controller: FPSCameraController,
    camera_entity: hecs::Entity,
}

impl CameraSystem {
    pub fn new(camera_entity: hecs::Entity) -> Self {
        Self {
            controller: FPSCameraController::new(Vec3::new(0.0, 0.0, 2.0)),
            camera_entity,
        }
    }

    pub fn update(&mut self, world: &mut World, dt: f32) {
        // After processing events, write camera pose back to ECS component.
        if let Ok(mut cam) = world.query_one_mut::<&mut Camera>(self.camera_entity) {
            cam.eye = self.controller.position;
            cam.center = self.controller.position + self.controller.front();
        }
    }
}

impl System<crate::Action> for CameraSystem {
    fn update(&mut self, world: &mut World, _assets: &AssetManager, _events: &mut raidillon_core::EventBus<crate::Action>, dt: f32) {
        self.update(world, dt);
    }
}

impl EventHandler<Action> for CameraSystem {
    fn handle(&mut self, event: &GameEvent<Action>) {
        match event {
            GameEvent::InputAction(action) => {
                match action {
                    Action::MoveForward => self.controller.position += self.controller.front() * 0.1,
                    Action::MoveBackward => self.controller.position -= self.controller.front() * 0.1,
                    Action::MoveLeft => {
                        let right = self.controller.front().cross(Vec3::Y).normalize();
                        self.controller.position -= right * 0.1;
                    }
                    Action::MoveRight => {
                        let right = self.controller.front().cross(Vec3::Y).normalize();
                        self.controller.position += right * 0.1;
                    }
                }
            }
            _ => {}
        }
    }
} 