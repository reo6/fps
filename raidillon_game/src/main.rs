use anyhow::Result;
use raidillon_render::{init_render_window, DisplayHandle};
use raidillon_ui::Gui;
mod engine;
use crate::engine::Engine;


fn main() -> Result<()> {
    let event_loop = winit::event_loop::EventLoop::builder()
        .build()
        .expect("create event-loop");

    let (window, display): (winit::window::Window, DisplayHandle) = init_render_window(&event_loop, "raidillon", (1280, 720))?;

    let mut engine = Engine::new(&display)?;

    let mut gui = Gui::new(&display, &window)?;

    event_loop
        .run(move |event, el| {
            use winit::event::{Event};

            gui.handle_event(&window, &event);

            engine.handle_event(&event);

            match event {
                Event::WindowEvent { event, .. } => {
                    if let winit::event::WindowEvent::CloseRequested = event {
                        el.exit();
                    }
                }
                Event::WindowEvent { .. } => {}
                Event::AboutToWait => {
                    engine.update();

                    // Render
                    let mut target = display.as_inner().draw();
                    engine.render_into(&mut target);
                    gui.render_with(&mut target, &window, |_| {});
                    target.finish().unwrap();

                    gui.prepare_frame(&window);
                    window.request_redraw();
                }
                _ => {}
            }
        })
        .map_err(Into::into)
}
