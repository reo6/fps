use std::collections::HashMap;
use raidillon_render::model::Model;
use glium::glutin::surface::WindowSurface;
use glium::Display;
use raidillon_render::gltf_loader;
use raidillon_ecs::ModelId;
use raidillon_render::render_system::ModelProvider;
use anyhow::Result;

pub struct AssetManager {
    models: Vec<Model>,
    model_cache: HashMap<String, ModelId>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self { models: Vec::new(), model_cache: HashMap::new() }
    }

    /// Load or retrieve a cached model, returning its `ModelId`.
    pub fn load_model<P: AsRef<str>>(&mut self, path: P, display: &Display<WindowSurface>) -> Result<ModelId> {
        let path_str = path.as_ref();
        if let Some(&id) = self.model_cache.get(path_str) {
            return Ok(id);
        }
        let model = gltf_loader::load_gltf(path_str, display)?;
        let id = ModelId(self.models.len());
        self.models.push(model);
        self.model_cache.insert(path_str.to_string(), id);
        Ok(id)
    }

    pub fn get_model(&self, id: ModelId) -> Option<&Model> {
        self.models.get(id.0)
    }
}

impl ModelProvider for AssetManager {
    fn get_model(&self, id: ModelId) -> Option<&Model> {
        self.get_model(id)
    }
} 