use raidillon_ecs::{Transform, ModelHandle};
use hecs::{Entity, World};
use crate::render::GliumRenderer;
use crate::model::Model;

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

    pub fn spawn_mesh(&mut self, model: Model, transform: Transform) -> Entity {
        let model_id = self.renderer.models.len();
        self.renderer.models.push(model);

        self.world.spawn((
            transform,
            ModelHandle(model_id),
        ))
    }

    pub fn despawn_mesh(&mut self, entity: Entity) {
        if let Ok(model_handle) = self.world.get::<&ModelHandle>(entity) {
            if model_handle.0 < self.renderer.models.len() {
                self.renderer.models.remove(model_handle.0);
            }
        }
        let _ = self.world.despawn(entity);
    }

    /// Render a single frame using the internal renderer & world.
    pub fn render(&mut self) {
        self.renderer.render(&self.world);
    }

    /// Render into an existing glium target surface. Useful for composing with
    /// other render passes (e.g. Dear ImGui).
    pub fn render_into<S: glium::Surface>(&mut self, target: &mut S) {
        self.renderer.render_into(&self.world, target);
    }
}
