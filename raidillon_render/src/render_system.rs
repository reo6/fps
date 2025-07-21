use hecs::World;
use raidillon_core::{AssetManager, ModelId};
use crate::render::GliumRenderer;
use crate::model::{Model, Material};
use crate::window::DisplayHandle;
use glium::Surface;

/// A pure render system that doesn't own the ECS world.
/// This decouples rendering from ECS world ownership.
pub struct RenderSystem {
    renderer: GliumRenderer,
    assets: AssetManager<Model, Material>,
}

impl RenderSystem {
    pub fn new(display: DisplayHandle) -> anyhow::Result<Self> {
        Ok(Self {
            renderer: GliumRenderer::new(display.as_inner().clone())?,
            assets: AssetManager::new(),
        })
    }

    pub fn render(&mut self, world: &World, target: &mut impl Surface) {
        // Pass the asset manager to the renderer for accessing models
        self.renderer.render_into_with_assets(world, &self.assets, target);
    }

    pub fn load_model(&mut self, path: &str) -> anyhow::Result<ModelId> {
        // Check cache first
        let model = crate::gltf_loader::load_gltf(path, self.renderer.display())?;
        let model_id = self.assets.cache_model(path.to_string(), Box::new(model));
        Ok(model_id)
    }

    pub fn display(&self) -> &glium::Display<glium::glutin::surface::WindowSurface> {
        self.renderer.display()
    }

    pub fn get_model(&self, id: ModelId) -> Option<&Model> {
        self.assets.get_model(id)
    }

    pub fn assets(&self) -> &AssetManager<Model, Material> {
        &self.assets
    }

    pub fn assets_mut(&mut self) -> &mut AssetManager<Model, Material> {
        &mut self.assets
    }
} 