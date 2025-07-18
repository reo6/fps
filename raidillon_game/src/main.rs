use anyhow::Result;
use glam::{Quat, Vec3, EulerRot};
use glium::backend::glutin::SimpleWindowBuilder;
use raidillon_core::Time;
use raidillon_ecs::Transform;
use raidillon_render::{Camera, GliumRenderer, gltf_loader, ECSRenderer};
use raidillon_ui::Gui;
use raidillon_input::{Input, FPSCameraController};
use winit::keyboard::KeyCode;
use winit::window::CursorGrabMode;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Action {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
}

fn main() -> Result<()> {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("create event-loop");

    let (window, display) = SimpleWindowBuilder::new()
        .with_title("raidillon")
        .with_inner_size(1280, 720)
        .build(&event_loop);

    // Create ECS renderer which internally owns both the world and the renderer
    let mut ecsr = {
        let world = hecs::World::new();
        let renderer = GliumRenderer::new(display.clone())?;
        ECSRenderer::new(renderer, world)
    };

    // Dear ImGui integration
    let mut gui = Gui::new(&display, &window)?;

    let mut input = Input::<Action>::new();
    input.map_key(KeyCode::KeyW, Action::MoveForward);
    input.map_key(KeyCode::KeyS, Action::MoveBackward);
    input.map_key(KeyCode::KeyA, Action::MoveLeft);
    input.map_key(KeyCode::KeyD, Action::MoveRight);

    let mut camera_controller = FPSCameraController::new(Vec3::new(0.0, 0.0, 2.0));

    let mut cursor_grabbed = false;
    let mut attempted_initial_grab = false;

    let mut time = Time::new();

    let object_ent = {
        let model_3d = gltf_loader::load_gltf("resources/models/tree.gltf", &display)?;
        ecsr.spawn_mesh(model_3d, Transform {
            translation: Vec3::new(0.0, -2.5, -5.0),
            rotation:    Quat::IDENTITY,
            scale:       Vec3::new(0.01, 0.01, 0.01),
        })
    };

    let ground_ent = {
        let model_3d = gltf_loader::load_gltf("resources/models/plane.gltf", &display)?;
        ecsr.spawn_mesh(model_3d, Transform {
            translation: Vec3::new(0.0, -1.5, 0.0),
            rotation:    Quat::IDENTITY,
            scale:       Vec3::new(1.0, 1.0, 1.0),
        })
    };


    let camera_ent = {
        let (w, h): (u32, u32) = window.inner_size().into();
        ecsr.world.spawn((Camera {
            eye:    Vec3::new(0.0, 0.0, 2.0),
            center: Vec3::ZERO,
            up:     Vec3::Y,
            fovy:   60_f32.to_radians(),
            aspect: w as f32 / h as f32,
            znear:  0.1,
            zfar:   100.0,
        },))
    };

    event_loop
        .run(move |event, el| {
            use glium::winit::event::{Event, WindowEvent};

            gui.handle_event(&window, &event);

            input.handle_event(&event);

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => el.exit(),
                    WindowEvent::Resized(sz) => {
                        ecsr.world.query_one_mut::<&mut Camera>(camera_ent).map(|mut cam| {
                            cam.aspect = sz.width as f32 / sz.height as f32;
                        });
                    }
                    WindowEvent::RedrawRequested => {
                        let mut target = display.draw();

                        ecsr.render_into(&mut target);

                        gui.render_with(&mut target, &window, |ui| {
                            if let Ok(mut tr) = ecsr.world.query_one_mut::<&mut Transform>(object_ent) {
                                // Translation controls
                                let mut translation = [tr.translation.x, tr.translation.y, tr.translation.z];
                                if ui.input_float3("Translation", &mut translation).build() {
                                    tr.translation = Vec3::from(translation);
                                }

                                // Scale controls
                                let mut scale = [tr.scale.x, tr.scale.y, tr.scale.z];
                                if ui.input_float3("Scale", &mut scale).build() {
                                    tr.scale = Vec3::from(scale);
                                }

                                // Rotation controls
                                let (yaw, pitch, roll) = tr.rotation.to_euler(EulerRot::YXZ);
                                let mut rotation_deg = [yaw.to_degrees(), pitch.to_degrees(), roll.to_degrees()];
                                if ui.input_float3("Rotation (deg)", &mut rotation_deg).build() {
                                    let yaw   = rotation_deg[0].to_radians();
                                    let pitch = rotation_deg[1].to_radians();
                                    let roll  = rotation_deg[2].to_radians();
                                    tr.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
                                }
                            }
                        });
                        target.finish().unwrap();
                    }
                    _ => {}
                },
                Event::AboutToWait => {
                    time.tick();

                    if !attempted_initial_grab {
                        attempted_initial_grab = true;
                        if window
                            .set_cursor_grab(CursorGrabMode::Confined)
                            .or_else(|_| window.set_cursor_grab(CursorGrabMode::Locked))
                            .is_ok()
                        {
                            window.set_cursor_visible(false);
                            cursor_grabbed = true;
                        }
                    }

                    {
                        let dt = time.delta_seconds();
                        camera_controller.update(
                            &input,
                            dt,
                            cursor_grabbed,
                            (Action::MoveForward, Action::MoveBackward, Action::MoveLeft, Action::MoveRight),
                        );

                        if input.key_pressed(KeyCode::Escape) {
                            if cursor_grabbed {
                                let _ = window.set_cursor_grab(CursorGrabMode::None);
                                window.set_cursor_visible(true);
                                cursor_grabbed = false;
                            } else if window
                                .set_cursor_grab(CursorGrabMode::Confined)
                                .or_else(|_| window.set_cursor_grab(CursorGrabMode::Locked))
                                .is_ok()
                            {
                                window.set_cursor_visible(false);
                                cursor_grabbed = true;
                            }
                        }

                        if let Ok(mut cam) = ecsr.world.query_one_mut::<&mut Camera>(camera_ent) {
                            cam.eye    = camera_controller.position;
                            cam.center = camera_controller.position + camera_controller.front();
                        }
                    }

                    input.end_frame();

                    gui.prepare_frame(&window);
                    window.request_redraw();
                }
                _ => {}
            }
        })
        .map_err(Into::into)
}
