mod camera;
mod ecs;
mod model;
mod render;
mod time;

use anyhow::Result;
use camera::Camera;
use ecs::{MeshHandle, Transform};
use glam::{Quat, Vec3};
use glium::backend::glutin::SimpleWindowBuilder;
use render::{Renderer, GliumRenderer};

fn main() -> Result<()> {
    const ROTATION_SPEED: f32 = 1.0;

    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("create event-loop");

    let (window, display) = SimpleWindowBuilder::new()
        .with_title("fps")
        .with_inner_size(1280, 720)
        .build(&event_loop);

    // Create ECS renderer which internally owns both the world and the renderer
    let mut ecsr = {
        let world = hecs::World::new();
        let renderer = GliumRenderer::new(display.clone())?;
        ecs::ECSRenderer::new(renderer, world)
    };

    let mut time = time::Time::new();

    let object_ent = {
        let mesh = model::load_gltf("resources/models/monkey.gltf", &display)?;
        ecsr.spawn_mesh(mesh, Transform {
                translation: Vec3::ZERO,
                rotation:    Quat::IDENTITY,
                scale:       Vec3::ONE,
        })
    };


    let camera_ent = {
        let (w, h): (u32, u32) = window.inner_size().into();
        ecsr.world.spawn((Camera {
            eye:    Vec3::new(3.0, 2.0, 3.0),
            center: Vec3::ZERO,
            up:     Vec3::Y,
            fovy:   45_f32.to_radians(),
            aspect: w as f32 / h as f32,
            znear:  0.1,
            zfar:   100.0,
        },))
    };

    event_loop
        .run(move |event, el| {
            use glium::winit::event::{Event, WindowEvent};

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => el.exit(),
                    WindowEvent::Resized(sz) => {
                        ecsr.world.query_one_mut::<&mut crate::camera::Camera>(camera_ent).map(|mut cam| {
                            cam.aspect = sz.width as f32 / sz.height as f32;
                        });
                    }
                    WindowEvent::RedrawRequested => {
                        ecsr.render();
                    }
                    _ => {}
                },
                Event::AboutToWait => {
                    time.tick();
                    let dt = time.delta_seconds();
                    ecsr.world.query_one_mut::<&mut Transform>(object_ent).map(|mut object| {
                        object.rotation *= Quat::from_rotation_y(ROTATION_SPEED * dt);
                    });

                    // despawn the object after 3 seconds
                    if time.total_seconds() > 3.0 {
                        ecsr.despawn_mesh(object_ent);
                    }

                    // ask for next frame
                    window.request_redraw();
                }
                _ => {}
            }
        })
        .map_err(Into::into)
}
