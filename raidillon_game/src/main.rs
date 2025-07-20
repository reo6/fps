use anyhow::Result;
use glam::{Quat, Vec3, EulerRot};
use raidillon_core::Time;
use raidillon_ecs::Transform;
use raidillon_render::{Camera, ECSRenderer, init_render_window, DisplayHandle};
use raidillon_ui::Gui;
use raidillon_input::{Input, FPSCameraController};
use raidillon_physics::{Physics, BodyKind, RigidBodyComponent};
use rapier3d::prelude::RigidBodyHandle;
use winit::keyboard::KeyCode;
use winit::window::CursorGrabMode;
use winit::event::MouseButton;
use nalgebra::vector;

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

    let (window, _display): (winit::window::Window, DisplayHandle) = init_render_window(&event_loop, "raidillon", (1920, 1080))?;

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

    // Physics world
    let mut physics = Physics::new();

    // Load a sphere model instead of the tree
    let sphere_ent = ecsr.load_mesh_from_gltf("resources/models/uvsphere-smooth.gltf", Transform {
        translation: Vec3::new(0.0, 2.5, 0.0),
        rotation:    Quat::IDENTITY,
        scale:       Vec3::new(0.5, 0.5, 0.5),
    })?;

    {
        let tr = *ecsr.world.get::<&Transform>(sphere_ent)?;
        let collider = rapier3d::prelude::ColliderBuilder::ball(0.5).build();
        let rb_handle = physics.add_rigid_body(BodyKind::Dynamic, tr, collider);
        ecsr.world.insert_one(sphere_ent, RigidBodyComponent(rb_handle))?;
    }

    let ground_ent = ecsr.load_mesh_from_gltf("resources/models/plane.gltf", Transform {
        translation: Vec3::new(0.0, -1.5, 0.0),
        rotation:    Quat::IDENTITY,
        scale:       Vec3::new(10.0, 1.0, 10.0),
    })?;

    {
        let tr = *ecsr.world.get::<&Transform>(ground_ent)?;
        let collider = rapier3d::prelude::ColliderBuilder::cuboid(10.0, 0.1, 10.0).build();
        let rb_handle = physics.add_rigid_body(BodyKind::Static, tr, collider);
        ecsr.world.insert_one(ground_ent, RigidBodyComponent(rb_handle))?;
    }

    let player_initial_tr = Transform {
        translation: Vec3::new(0.0, 1.0, 2.0),
        rotation:    Quat::IDENTITY,
        scale:       Vec3::ONE,
    };

    let player_collider = rapier3d::prelude::ColliderBuilder::capsule_y(0.9, 0.4).build();
    let player_rb_handle: RigidBodyHandle = physics.add_rigid_body(BodyKind::Dynamic, player_initial_tr, player_collider);
    if let Some(body) = physics.get_rigid_body_mut(player_rb_handle) {
        body.set_locked_axes(rapier3d::prelude::LockedAxes::ROTATION_LOCKED, true);
    }
    let _player_ent = ecsr.world.spawn((player_initial_tr, RigidBodyComponent(player_rb_handle)));

    camera_controller.position = player_initial_tr.translation;

    let camera_ent = {
        let (w, h): (u32, u32) = window.inner_size().into();
        ecsr.world.spawn((Camera {
            eye:    player_initial_tr.translation,
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
                            if let Ok(mut tr) = ecsr.world.query_one_mut::<&mut Transform>(sphere_ent) {
                                ui.text("Hold right click to control the camera");
                                ui.text("WASD to move");

                                static mut SHOW_COLLIDERS: bool = true;
                                unsafe {
                                    if ui.checkbox("Show Colliders", &mut SHOW_COLLIDERS) {
                                    }
                                    ecsr.renderer.set_show_colliders(SHOW_COLLIDERS);
                                }

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

                        let mut move_dir = Vec3::ZERO;
                        let front = camera_controller.front();
                        let front_h = Vec3::new(front.x, 0.0, front.z).normalize_or_zero();
                        let right_vec = front_h.cross(Vec3::Y).normalize_or_zero();

                        if input.action_held(Action::MoveForward)  { move_dir += front_h;     }
                        if input.action_held(Action::MoveBackward) { move_dir -= front_h;     }
                        if input.action_held(Action::MoveLeft)     { move_dir -= right_vec;   }
                        if input.action_held(Action::MoveRight)    { move_dir += right_vec;   }

                        if move_dir.length_squared() > 0.0 {
                            move_dir = move_dir.normalize();
                        }

                        if let Some(body) = physics.get_rigid_body_mut(player_rb_handle) {
                            let current_vel = body.linvel();
                            let desired_vel = move_dir * camera_controller.speed;
                            body.set_linvel(vector![desired_vel.x, current_vel.y, desired_vel.z], true);
                        }

                        physics.step(dt);

                        {
                            let mut query = ecsr.world.query::<(&mut Transform, &RigidBodyComponent)>();
                            for (_ent, (mut tr, rb_comp)) in query.iter() {
                                if let Some(body) = physics.get_rigid_body(rb_comp.0) {
                                    let pos = body.position();
                                    let translation = Physics::rapier_translation_to_glam(&pos.translation.vector);
                                    let rotation = Physics::rapier_rotation_to_glam(&pos.rotation);
                                    tr.translation = translation;
                                    tr.rotation = rotation;
                                }
                            }
                        }

                        if let Some(player_body) = physics.get_rigid_body(player_rb_handle) {
                            let eye = Physics::rapier_translation_to_glam(&player_body.position().translation.vector);
                            camera_controller.position = eye;

                            if let Ok(mut cam) = ecsr.world.query_one_mut::<&mut Camera>(camera_ent) {
                                cam.eye    = eye;
                                cam.center = eye + camera_controller.front();
                            }
                        }
                    }

                    input.end_frame();

                    gui.prepare_frame(&window);

                    ecsr.renderer.set_colliders(Some(&physics.collider_set));
                    window.request_redraw();
                }
                _ => {}
            }
        })
        .map_err(Into::into)
}
