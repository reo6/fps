use crate::camera::Camera;
use crate::ecs::{ModelHandle, Transform};
use crate::model::{Model};
use glium::texture::{RawImage2d, SrgbTexture2d};
use glium::{uniform, Program, Surface};
use glium::uniforms::{MinifySamplerFilter, MagnifySamplerFilter, SamplerWrapFunction};
use glam::Vec3;
use hecs::World;
use glium::glutin::surface::WindowSurface;

pub struct GliumRenderer {
    display: glium::Display<WindowSurface>,
    program: Program,
    white_tex: SrgbTexture2d,

    pub models: Vec<Model>,

    params: glium::DrawParameters<'static>,
}

impl GliumRenderer {
    pub fn new(display: glium::Display<WindowSurface>) -> anyhow::Result<Self> {
        const VERT_SRC: &str = include_str!("../resources/shaders/gl_textured.vert");
        const FRAG_SRC: &str = include_str!("../resources/shaders/gl_textured.frag");

        let program = Program::from_source(&display, VERT_SRC, FRAG_SRC, None)?;

        let white_tex = {
            let data = vec![255u8, 255u8, 255u8, 255u8];
            let raw  = RawImage2d::from_raw_rgba(data, (1, 1));
            SrgbTexture2d::new(&display, raw)?
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };
        
        Ok(Self {
            display,
            program,
            white_tex,
            models: Vec::new(),
            params,
        })
    }

    fn draw_scene<S: Surface>(&self, world: &World, target: &mut S) {
        let cam = match world.query::<&Camera>().iter().next() {
            Some((_, cam)) => *cam,
            None => {
                eprintln!("[renderer] No camera component found. Skipping frame");
                return;
            }
        };

        // Direction from the light source (0,+Y) towards the scene.
        let light_dir: Vec3 = Vec3::new(0.0, -1.0, 0.0).normalize();

        for (_, (tr, mh)) in world.query::<(&Transform, &ModelHandle)>().iter() {
            let model = &self.models[mh.0];
            let mesh  = &model.mesh;
            let mat   = &model.material;

            let tex_ref: &SrgbTexture2d = mat.base_color.as_ref().unwrap_or(&self.white_tex);

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
                color:      [c[0], c[1], c[2]],
                uv_offset:  [mat.uv_offset.x, mat.uv_offset.y],
                uv_scale:   [mat.uv_scale.x,  mat.uv_scale.y],
            };

            target.draw(
                &mesh.vbuf,
                &mesh.ibuf,
                &self.program,
                &uniforms,
                &self.params,
            ).unwrap();
        }
    }

    pub fn render_into<S: Surface>(&mut self, world: &World, target: &mut S) {
        target.clear_color_and_depth((0.1, 0.1, 0.15, 1.0), 1.0);
        self.draw_scene(world, target);
    }

    pub fn render(&mut self, world: &World) {
        let mut frame = self.display.draw();
        frame.clear_color_and_depth((0.1, 0.1, 0.15, 1.0), 1.0);
        self.draw_scene(world, &mut frame);
        frame.finish().unwrap();
    }
}
