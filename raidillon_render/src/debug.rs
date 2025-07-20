use glium::{implement_vertex, VertexBuffer, Surface, Program, DrawParameters, index::NoIndices, index::PrimitiveType, uniform};
use glium::glutin::surface::WindowSurface;
use rapier3d::prelude::{ColliderSet, Aabb};
use glam::{Mat4};

#[derive(Copy, Clone)]
pub struct DebugVertex {
    position: [f32; 3],
}

implement_vertex!(DebugVertex, position);

pub struct ColliderDebugRenderer {
    program: Program,
    display: glium::Display<WindowSurface>,
}

impl ColliderDebugRenderer {
    pub fn new(display: &glium::Display<WindowSurface>) -> anyhow::Result<Self> {
        const VERT: &str = r#"#version 330 core
layout(location = 0) in vec3 position;
uniform mat4 vp;
void main() {
   gl_Position = vp * vec4(position, 1.0);
}
"#;
        const FRAG: &str = r#"#version 330 core
out vec4 color;
void main() {
    color = vec4(1.0,1.0,0.0,1.0);
}
"#;
        let program = Program::from_source(display, VERT, FRAG, None)?;
        Ok(Self { program, display: display.clone() })
    }

    pub fn draw<S: Surface>(&self, colliders: &ColliderSet, vp: Mat4, target: &mut S) {
        let mut vertices: Vec<DebugVertex> = Vec::new();
        for (_, c) in colliders.iter() {
            let aabb: Aabb = c.compute_aabb();
            let min = aabb.mins;
            let max = aabb.maxs;
            // 8 corners
            let p0 = [min.x, min.y, min.z];
            let p1 = [max.x, min.y, min.z];
            let p2 = [max.x, max.y, min.z];
            let p3 = [min.x, max.y, min.z];
            let p4 = [min.x, min.y, max.z];
            let p5 = [max.x, min.y, max.z];
            let p6 = [max.x, max.y, max.z];
            let p7 = [min.x, max.y, max.z];
            // 12 edges (pairs)
            let edges = [
                (p0, p1), (p1, p2), (p2, p3), (p3, p0),
                (p4, p5), (p5, p6), (p6, p7), (p7, p4),
                (p0, p4), (p1, p5), (p2, p6), (p3, p7),
            ];
            for (a, b) in edges.iter() {
                vertices.push(DebugVertex { position: *a });
                vertices.push(DebugVertex { position: *b });
            }
        }
        if vertices.is_empty() { return; }
        let vb = VertexBuffer::new(&self.display, &vertices).unwrap();
        let no_indices = NoIndices(PrimitiveType::LinesList);
        let uniforms = uniform! { vp: vp.to_cols_array_2d() };
        let params = DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLessOrEqual,
                write: false,
                .. Default::default()
            },
            polygon_mode: glium::draw_parameters::PolygonMode::Line,
            line_width: Some(1.0),
            .. Default::default()
        };
        target.draw(&vb, &no_indices, &self.program, &uniforms, &params).ok();
    }
} 
