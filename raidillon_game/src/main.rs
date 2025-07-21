use anyhow::Result;
use glam::{Quat, Vec3, EulerRot};
use raidillon_core::{Time, EventBus, GameEvent, InputAction, System, SystemRegistry, AssetManager, Model, Material};
use raidillon_ecs::Transform;
use raidillon_render::{RenderSystem, init_render_window, DisplayHandle};
use raidillon_ui::Gui;
use raidillon_input::{InputSystem, CameraSystem};
use raidillon_game::GameState;
use winit::window::CursorGrabMode;
use winit::event::MouseButton;
use hecs::World;

// Wrapper to make RenderSystem implement the System trait
struct RenderSystemWrapper {
    render_system: RenderSystem,
}

impl RenderSystemWrapper {
    fn new(display: DisplayHandle) -> anyhow::Result<Self> {
        Ok(Self {
            render_system: RenderSystem::new(display)?,
        })
    }

    fn load_model(&mut self, path: &str) -> anyhow::Result<raidillon_core::ModelId> {
        self.render_system.load_model(path)
    }

    fn render(&mut self, world: &World, target: &mut impl glium::Surface) {
        self.render_system.render(world, target)
    }

    fn display(&self) -> &glium::Display<glium::glutin::surface::WindowSurface> {
        self.render_system.display()
    }
}

impl System for RenderSystemWrapper {
    fn update(&mut self, _world: &mut World, _resources: &AssetManager<dyn Model, dyn Material>, _events: &mut EventBus, _dt: f32) {
        // Rendering is handled separately in the main loop
    }

    fn handle_event(&mut self, _event: &GameEvent, _world: &mut World) {
        // RenderSystem doesn't need to respond to events currently
    }

    fn name(&self) -> &'static str {
        "RenderSystem"
    }
}

fn main() -> Result<()> {
    let event_loop = winit::event_loop::EventLoop::builder()
        .build()
        .expect("create event-loop");

    let (window, _display): (winit::window::Window, DisplayHandle) = init_render_window(&event_loop, "raidillon", (1280, 720))?;

    // Create game state and systems
    let mut game_state = GameState::new();
    let mut render_system_wrapper = RenderSystemWrapper::new(_display.clone())?;

    // Dear ImGui integration
    let mut gui = Gui::new(&_display, &window)?;

    // Create system registry and register systems
    let mut system_registry = SystemRegistry::new();
    let mut event_bus = EventBus::new();
    let mut input_system = InputSystem::new(); // Keep this for direct access
    let mut camera_system = CameraSystem::new(game_state.camera_entity); // Keep this for direct access

    // Register systems later when we have proper asset manager integration
    // For now, manage systems directly in main loop

    let mut right_mouse_held = false;
    let mut time = Time::new();

    // Load models using the RenderSystem
    let object_model_id = render_system_wrapper.load_model("resources/models/tree.gltf")?;
    let ground_model_id = render_system_wrapper.load_model("resources/models/plane.gltf")?;

    // Update the model handles in game state using the new method
    game_state.update_entity_model(game_state.object_entity, object_model_id)?;
    game_state.update_entity_model(game_state.ground_entity, ground_model_id)?;

    // Set initial camera aspect ratio
    let (w, h): (u32, u32) = window.inner_size().into();
    game_state.resize_camera(w, h);

    event_loop
        .run(move |event, el| {
            use winit::event::{Event, WindowEvent};

            gui.handle_event(&window, &event);
            input_system.handle_event(&event);

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => el.exit(),
                    WindowEvent::Resized(sz) => {
                        camera_system.resize_camera(game_state.world_mut(), sz.width, sz.height);
                        event_bus.emit(GameEvent::WindowResize { width: sz.width, height: sz.height });
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
                        // First render the 3D world
                        let mut target = render_system_wrapper.display().draw();
                        render_system_wrapper.render(game_state.world(), &mut target);

                        // Then overlay ImGui on top
                        gui.render_with(&mut target, &window, |ui| {
                            let object_entity = game_state.object_entity;
                            if let Ok(tr) = game_state.world_mut().query_one_mut::<&mut Transform>(object_entity) {
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

                        target.finish().expect("Failed to swap buffers");
                    }
                    _ => {}
                },
                Event::AboutToWait => {
                    time.tick();
                    let dt = time.delta_seconds();
                    
                    // Update input system and generate events
                    input_system.update(&mut event_bus, right_mouse_held);
                    
                    // Process input events for camera movement
                    let mouse_delta = input_system.mouse_delta();
                    
                    // Handle camera input actions
                    for event in event_bus.events() {
                        if let GameEvent::InputAction(action) = event {
                            camera_system.handle_input_action(*action, dt);
                        }
                    }
                    
                    // Update camera with mouse movement
                    camera_system.update(game_state.world_mut(), dt, mouse_delta);
                    
                    // Update game state
                    game_state.update(dt);

                    // Process all events
                    event_bus.process();

                    input_system.end_frame();
                    gui.prepare_frame(&window);
                    window.request_redraw();
                }
                _ => {}
            }
        })
        .map_err(Into::into)
}
