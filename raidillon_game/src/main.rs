use anyhow::Result;
use glam::{Quat, Vec3, EulerRot};
use raidillon_core::Time;
use raidillon_ecs::Transform;
use raidillon_render::{Camera, ECSRenderer, init_render_window, DisplayHandle};
use raidillon_ui::Gui;
use raidillon_input::{Input, FPSCameraController};
use winit::keyboard::KeyCode;
use winit::window::CursorGrabMode;
use winit::event::MouseButton;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Action {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
}

fn main() -> Result<()> {
    let event_loop = winit::event_loop::EventLoop::builder()
        .build()
        .expect("create event-loop");

    let (window, _display): (winit::window::Window, DisplayHandle) = init_render_window(&event_loop, "raidillon", (1280, 720))?;

    // Create ECS renderer which internally owns both the world and the renderer
    let mut ecsr = ECSRenderer::from_display_handle(&_display)?;

    // Dear ImGui integration
    let mut gui = Gui::new(&_display, &window)?;

    let mut input = Input::<Action>::new();
    input.map_key(KeyCode::KeyW, Action::MoveForward);
    input.map_key(KeyCode::KeyS, Action::MoveBackward);
    input.map_key(KeyCode::KeyA, Action::MoveLeft);
    input.map_key(KeyCode::KeyD, Action::MoveRight);

    let mut camera_controller = FPSCameraController::new(Vec3::new(0.0, 0.0, 2.0));

    let mut right_mouse_held = false;

    let mut time = Time::new();

    let object_ent = ecsr.load_mesh_from_gltf("resources/models/tree.gltf", Transform {
        translation: Vec3::new(0.0, -2.5, -5.0),
        rotation:    Quat::IDENTITY,
        scale:       Vec3::new(0.01, 0.01, 0.01),
    })?;

    let ground_ent = ecsr.load_mesh_from_gltf("resources/models/plane.gltf", Transform {
        translation: Vec3::new(0.0, -1.5, 0.0),
        rotation:    Quat::IDENTITY,
        scale:       Vec3::new(1.0, 1.0, 1.0),
    })?;


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
            use winit::event::{Event, WindowEvent};

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
                    WindowEvent::MouseInput { state, button, .. } => {
                        if button == MouseButton::Right {
                            match state {
                                winit::event::ElementState::Pressed => {
                                    if window
                                        .set_cursor_grab(CursorGrabMode::Confined)
                                        .or_else(|_| window.set_cursor_grab(CursorGrabMode::Locked))
                                        .is_ok()
                                    {
                                        window.set_cursor_visible(false);
                                        right_mouse_held = true;
                                    }
                                }
                                winit::event::ElementState::Released => {
                                    let _ = window.set_cursor_grab(CursorGrabMode::None);
                                    window.set_cursor_visible(true);
                                    right_mouse_held = false;
                                }
                            }
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        gui.render_world(&mut ecsr, &window, |ui, ecsr| {
                            if let Ok(mut tr) = ecsr.world.query_one_mut::<&mut Transform>(object_ent) {
                                ui.text("Hold right click to control the camera");
                                ui.text("WASD to move");

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
                    }
                    _ => {}
                },
                Event::AboutToWait => {
                    time.tick();

                    {
                        let dt = time.delta_seconds();
                        camera_controller.update(
                            &input,
                            dt,
                            right_mouse_held,
                            (Action::MoveForward, Action::MoveBackward, Action::MoveLeft, Action::MoveRight),
                        );

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
