use glam::{Mat4, Quat, Vec3};
use hecs::{Entity, World};
use crate::{render::{GliumRenderer, Renderer}, model};

/// This system joins the renderer and ECS,
/// and provides tools to use them together
/// effectively.
pub struct ECSRenderer {
    pub renderer: GliumRenderer,
    pub world: World,
}

impl ECSRenderer {
    pub fn new(renderer: GliumRenderer, world: World) -> Self {
        Self { renderer, world }
    }

    pub fn spawn_mesh(&mut self, mesh: model::Mesh, transform: Transform) -> Entity {
        let mesh_id = self.renderer.meshes.len();
        self.renderer.meshes.push(mesh);
        self.world.spawn((transform, MeshHandle(mesh_id)))
    }

    pub fn despawn_mesh(&mut self, entity: Entity) {
        if let Ok(mesh_handle) = self.world.get::<&MeshHandle>(entity) {
            if mesh_handle.0 < self.renderer.meshes.len() {
                self.renderer.meshes.remove(mesh_handle.0);
            }
        }
        let _ = self.world.despawn(entity);
    }

    /// Render a single frame using the internal renderer & world.
    pub fn render(&mut self) {
        self.renderer.render(&self.world);
    }
}

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
