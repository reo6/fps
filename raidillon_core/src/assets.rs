use std::collections::HashMap;

// Forward declarations - these will be from other crates
pub trait Model {}
pub trait Material {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ModelId(pub usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MaterialId(pub usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TextureHandle(pub usize);

pub struct AssetManager<M: Model + ?Sized, Mat: Material + ?Sized> {
    models: Vec<Box<M>>,
    materials: Vec<Box<Mat>>,
    textures: HashMap<String, TextureHandle>,
    model_cache: HashMap<String, ModelId>,
    next_texture_id: usize,
}

impl<M: Model + ?Sized, Mat: Material + ?Sized> AssetManager<M, Mat> {
    pub fn new() -> Self {
        Self {
            models: Vec::new(),
            materials: Vec::new(),
            textures: HashMap::new(),
            model_cache: HashMap::new(),
            next_texture_id: 0,
        }
    }

    pub fn add_model(&mut self, model: Box<M>) -> ModelId {
        let id = ModelId(self.models.len());
        self.models.push(model);
        id
    }

    pub fn cache_model(&mut self, path: String, model: Box<M>) -> ModelId {
        if let Some(&cached_id) = self.model_cache.get(&path) {
            return cached_id;
        }
        
        let model_id = self.add_model(model);
        self.model_cache.insert(path, model_id);
        model_id
    }

    pub fn get_model(&self, id: ModelId) -> Option<&M> {
        self.models.get(id.0).map(|boxed| boxed.as_ref())
    }

    pub fn get_model_mut(&mut self, id: ModelId) -> Option<&mut M> {
        self.models.get_mut(id.0).map(|boxed| boxed.as_mut())
    }

    pub fn add_material(&mut self, material: Box<Mat>) -> MaterialId {
        let id = MaterialId(self.materials.len());
        self.materials.push(material);
        id
    }

    pub fn get_material(&self, id: MaterialId) -> Option<&Mat> {
        self.materials.get(id.0).map(|boxed| boxed.as_ref())
    }

    pub fn add_texture(&mut self, name: String) -> TextureHandle {
        if let Some(&handle) = self.textures.get(&name) {
            return handle;
        }
        
        let handle = TextureHandle(self.next_texture_id);
        self.next_texture_id += 1;
        self.textures.insert(name, handle);
        handle
    }

    pub fn get_texture_handle(&self, name: &str) -> Option<TextureHandle> {
        self.textures.get(name).copied()
    }

    pub fn model_count(&self) -> usize {
        self.models.len()
    }

    pub fn material_count(&self) -> usize {
        self.materials.len()
    }

    pub fn clear_cache(&mut self) {
        self.model_cache.clear();
    }
} 