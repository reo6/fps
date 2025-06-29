mod camera;
mod ecs;
mod model;
mod render;

use anyhow::Result;
use camera::Camera;
use ecs::{rotation_system, MeshHandle, Transform};
use glam::{Quat, Vec3};
use glium::backend::glutin::SimpleWindowBuilder;
use render::{Renderer, GliumRenderer};
use std::time::Instant;

fn main() -> Result<()> {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("create event-loop");

    let (window, display) = SimpleWindowBuilder::new()
        .with_title("fps")
        .with_inner_size(1280, 720)
        .build(&event_loop);

    let mut world = hecs::World::new();

    let mesh         = model::load_gltf("resources/monkey-smooth.gltf", &display)?;
    // let mesh         = model::cube(&display)?;
    let mut renderer = GliumRenderer::new(display)?;
    let mesh_id      = renderer.meshes.len();
    renderer.meshes.push(mesh);

    world.spawn((
        Transform {
            translation: Vec3::ZERO,
            rotation:    Quat::IDENTITY,
            scale:       Vec3::ONE,
        },
        MeshHandle(mesh_id),
    ));

    {
        let (w, h): (u32, u32) = window.inner_size().into();
        world.spawn((Camera {
            eye:    Vec3::new(3.0, 2.0, 3.0),
            center: Vec3::ZERO,
            up:     Vec3::Y,
            fovy:   45_f32.to_radians(),
            aspect: w as f32 / h as f32,
            znear:  0.1,
            zfar:   100.0,
        },));
    }

    event_loop
        .run(move |event, el| {
            use glium::winit::event::{Event, WindowEvent};

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => el.exit(),
                    WindowEvent::Resized(sz) => {
                        ecs::set_camera_aspect(&mut world, sz.width as f32 / sz.height as f32);
                    }
                    WindowEvent::RedrawRequested => {
                        renderer.render(&world);
                    }
                    _ => {}
                },
                Event::AboutToWait => {
                    // -- update logic --
                    let now = Instant::now();
                    static mut LAST: Option<Instant> = None;
                    let dt = unsafe { // FIXME
                        let last = LAST.replace(now).unwrap_or(now);
                        (now - last).as_secs_f32()
                    };
                    rotation_system(&mut world, dt);

                    // ask for next frame
                    window.request_redraw();
                }
                _ => {}
            }
        })
        .map_err(Into::into)
}
