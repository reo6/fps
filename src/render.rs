use crate::camera::Camera;
use crate::ecs::{MeshHandle, Transform};
use crate::model::Mesh;
use glium::{uniform, Program, Surface};
use glam::Vec3;
use hecs::World;
use glium::glutin::surface::WindowSurface;

/// Generic rendering backend trait.
pub trait Renderer {
    /// Render a single frame for the given `World`.
    fn render(&mut self, world: &World);
}

/// Concrete OpenGL (glium) renderer implementing `Renderer`.
pub struct GliumRenderer {
    display: glium::Display<WindowSurface>,
    program: Program,
    pub meshes: Vec<Mesh>,
    params: glium::DrawParameters<'static>,
}

impl GliumRenderer {
    /// Create a new OpenGL renderer consuming the provided `display`.
    pub fn new(display: glium::Display<WindowSurface>) -> anyhow::Result<Self> {
        const VERT: &str = r#"
            #version 330 core
            in  vec3 position;
            in  vec3 normal;
            uniform mat4 model;
            uniform mat4 view;
            uniform mat4 projection;
            out vec3 v_normal;
            void main() {
                mat4 modelview = view * model;
                v_normal = transpose(inverse(mat3(modelview))) * normal;
                gl_Position = projection * modelview * vec4(position, 1.0);
            }"#;

        const FRAG: &str = r#"
            #version 330 core
            in  vec3 v_normal;
            out vec4 color;
            uniform vec3 light_dir;

            void main() {
                float brightness = dot(normalize(v_normal), normalize(light_dir));
                vec3 dark_color = vec3(0.6, 0.0, 0.0);
                vec3 regular_color = vec3(1.0, 0.0, 0.0);
                color = vec4(mix(dark_color, regular_color, brightness), 1.0);
            }"#;



        let program = Program::from_source(&display, VERT, FRAG, None)?;

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };
        
        Ok(Self { display, program, meshes: Vec::new(), params })
    }
}

impl Renderer for GliumRenderer {
    fn render(&mut self, world: &World) {
        let mut frame = self.display.draw();
        frame.clear_color_and_depth((0.1, 0.1, 0.15, 1.0), 1.0);

        let cam = match world.query::<&Camera>().iter().next() {
            Some((_, cam)) => *cam,
            None => {
                eprintln!("[renderer] No camera component found – skipping frame");
                return;
            }
        };

        let light_dir: Vec3 = Vec3::new(-1.0, -1.0, -1.0).normalize();

        for (_, (tr, mh)) in world.query::<(&Transform, &MeshHandle)>().iter() {
            let mesh = &self.meshes[mh.0];
            let uniforms = uniform! {
                model:      tr.matrix().to_cols_array_2d(),
                view:       cam.view().to_cols_array_2d(),
                projection: cam.projection().to_cols_array_2d(),
                light_dir:  [light_dir.x, light_dir.y, light_dir.z],
            };

            frame.draw(&mesh.vbuf, &mesh.ibuf, &self.program, &uniforms, &self.params)
                 .unwrap();
        }

        frame.finish().unwrap();
    }
}
