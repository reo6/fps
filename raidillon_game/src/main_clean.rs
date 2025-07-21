use anyhow::Result;
use raidillon_core::{Engine, GameEvent};
use raidillon_render::{init_render_window, RenderSystem};
use raidillon_ui::Gui;
use raidillon_input::{InputSystem, CameraSystem};
use winit::event::{Event, WindowEvent};
use winit::window::CursorGrabMode;
use winit::event::MouseButton;

fn main() -> Result<()> {
    let event_loop = winit::event_loop::EventLoop::builder()
        .build()
        .expect("create event-loop");

    let (window, _display) = init_render_window(&event_loop, "raidillon", (1280, 720))?;

    // Create the unified engine
    let mut engine = Engine::new();
    
    // Create render system separately (for now, until full integration)
    let mut render_system = RenderSystem::new(_display.clone())?;
    
    // Create GUI system
    let mut gui = Gui::new(&_display, &window)?;

    // Add systems to the engine
    let input_system = InputSystem::new();
    let camera_system = CameraSystem::new(engine.world().spawn(())); // placeholder camera entity
    
    engine.add_system(input_system);
    engine.add_system(camera_system);

    // Load initial scene content
    load_default_scene(&mut engine, &mut render_system)?;

    let mut right_mouse_held = false;

    event_loop
        .run(move |event, el| {
            gui.handle_event(&window, &event);

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => el.exit(),
                    WindowEvent::Resized(sz) => {
                        engine.handle_window_event(&GameEvent::WindowResize { 
                            width: sz.width, 
                            height: sz.height 
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
                        // Update engine
                        engine.update();

                        // Render
                        let mut target = render_system.display().draw();
                        render_system.render(engine.world(), &mut target);
                        
                        // Render debug UI
                        gui.render_with(&mut target, &window, |ui| {
                            render_debug_ui(&engine, ui);
                        });

                        target.finish().expect("Failed to swap buffers");
                    }
                    _ => {}
                },
                Event::AboutToWait => {
                    gui.prepare_frame(&window);
                    window.request_redraw();
                }
                _ => {}
            }
        })
        .map_err(Into::into)
}

fn load_default_scene(engine: &mut Engine, render_system: &mut RenderSystem) -> Result<()> {
    // Load and setup default scene
    let _tree_model = render_system.load_model("resources/models/tree.gltf")?;
    let _ground_model = render_system.load_model("resources/models/plane.gltf")?;
    
    // Note: Full integration would require coordinating between engine and render system
    // For now, this demonstrates the clean architecture structure
    
    println!("Loaded default scene with {} systems", engine.system_count());
    Ok(())
}

fn render_debug_ui(engine: &Engine, ui: &imgui::Ui) {
    ui.text(format!("Engine Systems: {}", engine.system_count()));
    ui.text(format!("Delta Time: {:.3}ms", engine.delta_time() * 1000.0));
    ui.text("Clean Architecture Demo");
    ui.text("This shows the unified Engine approach");
} 