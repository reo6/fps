use glium::backend::glutin::SimpleWindowBuilder;
use glium::glutin::surface::WindowSurface;
use glium::Display;
use anyhow::Result;
use winit::event_loop::EventLoop;
use winit::window::Window;

#[derive(Clone)]
pub struct DisplayHandle(Display<WindowSurface>);

impl DisplayHandle {
    pub fn as_inner(&self) -> &Display<WindowSurface> {
        &self.0
    }
}

pub fn init_window<T>(
    event_loop: &EventLoop<T>,
    title: &str,
    size: (u32, u32),
) -> Result<(Window, DisplayHandle)> {
    let (window, display) = SimpleWindowBuilder::new()
        .with_title(title)
        .with_inner_size(size.0, size.1)
        .build(event_loop);

    Ok((window, DisplayHandle(display)))
}
