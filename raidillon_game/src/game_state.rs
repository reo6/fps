use anyhow;
use glam::{Quat, Vec3};
use hecs::{Entity, World};
use raidillon_ecs::{Transform, ModelHandle};
use raidillon_render::{Camera, ModelId};
use raidillon_core::InputAction;

pub struct GameState {
    pub world: World,
    pub camera_entity: Entity,
    pub object_entity: Entity,
    pub ground_entity: Entity,
}

impl GameState {
    pub fn new() -> Self {
        let mut world = World::new();
        
        // Create camera entity
        let camera_entity = world.spawn((Camera {
            eye: Vec3::new(0.0, 0.0, 2.0),
            center: Vec3::ZERO,
            up: Vec3::Y,
            fovy: 60_f32.to_radians(),
            aspect: 1280.0 / 720.0, // default aspect ratio
            znear: 0.1,
            zfar: 100.0,
        },));

        // Create placeholder entities for object and ground (will be properly loaded later)
        let object_entity = world.spawn((
            Transform {
                translation: Vec3::new(0.0, -2.5, -5.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::new(0.01, 0.01, 0.01),
            },
            ModelHandle(0),
        ));

        let ground_entity = world.spawn((
            Transform {
                translation: Vec3::new(0.0, -1.5, 0.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::new(1.0, 1.0, 1.0),
            },
            ModelHandle(1),
        ));

        Self {
            world,
            camera_entity,
            object_entity,
            ground_entity,
        }
    }

    pub fn update(&mut self, _dt: f32) {
        // Game state update logic will go here
        // Camera updates are now handled by CameraSystem
    }

    pub fn resize_camera(&mut self, width: u32, height: u32) {
        if let Ok(cam) = self.world.query_one_mut::<&mut Camera>(self.camera_entity) {
            cam.aspect = width as f32 / height as f32;
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn spawn_model(&mut self, model_id: ModelId, transform: Transform) -> Entity {
        self.world.spawn((transform, ModelHandle(model_id.0)))
    }

    pub fn update_entity_model(&mut self, entity: Entity, model_id: ModelId) -> anyhow::Result<()> {
        if let Ok(model_handle) = self.world.query_one_mut::<&mut ModelHandle>(entity) {
            model_handle.0 = model_id.0;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Entity does not have a ModelHandle component"))
        }
    }
} 