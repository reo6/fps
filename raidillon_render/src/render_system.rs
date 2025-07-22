use crate::render::GliumRenderer;
use glium::Surface;
use hecs::World;
use raidillon_ecs::ModelId;
use crate::model::Model;

pub trait ModelProvider {
    fn get_model(&self, id: ModelId) -> Option<&Model>;
}

/// Pure render system that owns the low-level renderer but **not** the ECS
/// world, allowing it to be plugged into any external world.
pub struct RenderSystem {
    renderer: GliumRenderer,
}

impl RenderSystem {
    /// Construct a RenderSystem from a window `DisplayHandle`.
    pub fn new(display: crate::window::DisplayHandle) -> anyhow::Result<Self> {
        Ok(Self {
            renderer: GliumRenderer::new(display.as_inner().clone())?,
        })
    }

    /// Render the given `world` into an arbitrary glium surface.
    pub fn render_into<S: Surface, P: ModelProvider>(&mut self, world: &World, assets: &P, target: &mut S) {
        // delegate to custom draw that uses assets
        self.draw_scene(world, assets, target);
    }

    pub fn render<S: Surface, P: ModelProvider>(&mut self, world: &World, assets: &P) {
        let mut frame = self.renderer.display().draw();
        self.draw_scene(world, assets, &mut frame);
        frame.finish().unwrap();
    }

    /// Load model via AssetManager caching.
    pub fn load_model<P: AsRef<str>, A: ModelProvider + ?Sized>( &self, path: P, assets: &mut A ) -> anyhow::Result<ModelId> where A: crate::render_system::ModelProvider {
        // cannot implement generic load here without knowing concrete; will leave stub not used.
        anyhow::bail!("Not implemented - load via AssetManager in core");
    }

    /// Expose the underlying display (useful for ImGui, etc.).
    pub fn display(&self) -> &glium::Display<glium::glutin::surface::WindowSurface> {
        self.renderer.display()
    }

    fn draw_scene<S: Surface, P: ModelProvider>(&self, world: &World, assets: &P, target: &mut S) {
        // replicate old GliumRenderer::draw_scene but using assets
        use glium::{uniform, uniforms::{MinifySamplerFilter, MagnifySamplerFilter, SamplerWrapFunction}};
        use glam::{Vec3, Vec4};
        use raidillon_ecs::{Transform};

        let cam = match world.query::<&crate::camera::Camera>().iter().next() {
            Some((_, cam)) => *cam,
            None => return,
        };

        let light_dir: Vec3 = Vec3::new(0.0, -1.0, 0.0).normalize();

        for (_, (tr, mh)) in world.query::<(&Transform, &ModelId)>().iter() {
            if let Some(model) = assets.get_model(*mh) {
                let mesh = &model.mesh;
                let mat  = &model.material;

                let tex_ref = mat.base_color.as_ref().unwrap_or(&self.renderer.white_tex);

                let mut sampler = tex_ref.sampled();
                sampler = sampler.wrap_function(SamplerWrapFunction::Repeat);
                sampler = sampler.minify_filter(MinifySamplerFilter::Linear);
                sampler = sampler.magnify_filter(MagnifySamplerFilter::Linear);

                let c = mat.base_color_factor;

                let uniforms = uniform! {
                    model:      tr.matrix().to_cols_array_2d(),
                    view:       cam.view().to_cols_array_2d(),
                    projection: cam.projection().to_cols_array_2d(),
                    u_light:    [light_dir.x, light_dir.y, light_dir.z],
                    tex:        sampler,
                    color:      [c[0],c[1],c[2]],
                    uv_offset:  [mat.uv_offset.x, mat.uv_offset.y],
                    uv_scale:   [mat.uv_scale.x, mat.uv_scale.y],
                };

                target.draw(&mesh.vbuf, &mesh.ibuf, &self.renderer.program, &uniforms, &self.renderer.params).unwrap();
            }
        }

        // skybox omitted for brevity
    }
} 