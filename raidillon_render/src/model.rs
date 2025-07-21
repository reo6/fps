use glium::texture::{RawImage2d, SrgbTexture2d, Texture2d};
use glium::uniforms::SamplerBehavior;
use glam::{Vec2};
use glium::{backend::Facade, implement_vertex, IndexBuffer, VertexBuffer};
use glium::index::PrimitiveType;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position:   [f32; 3],
    pub normal:     [f32; 3],
    pub tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, normal, tex_coords);

pub struct Mesh {
    pub vbuf: VertexBuffer<Vertex>,
    pub ibuf: IndexBuffer<u32>,
}

pub struct Material {
    pub base_color:         Option<SrgbTexture2d>,
    pub metallic_roughness: Option<Texture2d>,
    pub normal:             Option<Texture2d>,
    pub occlusion:          Option<Texture2d>,
    pub emissive:           Option<SrgbTexture2d>,
    pub sampler: SamplerBehavior,
    pub uv_offset: Vec2,
    pub uv_scale:  Vec2,
    pub base_color_factor:   [f32; 4],
    pub emissive_factor:     [f32; 3],
    pub metal_factor:        f32,
    pub roughness_factor:    f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            base_color: None,
            metallic_roughness: None,
            normal: None,
            occlusion: None,
            emissive: None,
            sampler: SamplerBehavior::default(),
            uv_offset: Vec2::ZERO,
            uv_scale:  Vec2::ONE,
            base_color_factor: [1.0; 4],
            emissive_factor:   [0.0; 3],
            metal_factor:      1.0,
            roughness_factor:  1.0,
        }
    }
}

// Implement the Material trait from raidillon_core
impl raidillon_core::Material for Material {}

pub struct Model {
    pub mesh:     Mesh,
    pub material: Material,
}

// Implement the Model trait from raidillon_core
impl raidillon_core::Model for Model {}
