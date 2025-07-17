use std::time::Instant;

use anyhow::Result;
use imgui::{Context as ImguiContext, Ui};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use imgui_glium_renderer::Renderer as ImguiGliumRenderer;
use winit::window::Window;
use glium::{Frame};
use glium::glutin::surface::WindowSurface;

/// Convenience wrapper that owns all ImGui state required for integration with
/// winit + glium.
pub struct Gui {
    imgui:      ImguiContext,
    platform:   WinitPlatform,
    renderer:   ImguiGliumRenderer,
    last_frame: Instant,
}

impl Gui {
    pub fn new(display: &glium::Display<WindowSurface>, window: &Window) -> Result<Self> {
        let mut imgui = ImguiContext::create();
        imgui.set_ini_filename(None);
        let mut platform = WinitPlatform::new(&mut imgui);
        platform.attach_window(imgui.io_mut(), window, HiDpiMode::Default);
        imgui.fonts().add_font(&[imgui::FontSource::DefaultFontData { config: None }]);
        let renderer = ImguiGliumRenderer::new(&mut imgui, display)?;

        Ok(Self {
            imgui,
            platform,
            renderer,
            last_frame: Instant::now(),
        })
    }

    pub fn handle_event<T>(&mut self, window: &Window, event: &winit::event::Event<T>) {
        self.platform
            .handle_event(self.imgui.io_mut(), window, event);
    }

    pub fn prepare_frame(&mut self, window: &Window) {
        let now = Instant::now();
        self.imgui.io_mut().update_delta_time(now - self.last_frame);
        self.last_frame = now;

        self
            .platform
            .prepare_frame(self.imgui.io_mut(), window)
            .expect("failed to prepare imgui frame");
    }

    pub fn render(&mut self, target: &mut Frame, window: &Window) {
        let mut open = true;
        self.render_with(target, window, |ui| {
            ui.show_demo_window(&mut open);
        });
    }

    pub fn render_with<F>(&mut self, target: &mut Frame, window: &Window, build_ui: F)
    where
        F: FnOnce(&Ui),
    {
        let ui = self.imgui.frame();

        build_ui(&ui);

        self.platform.prepare_render(ui, window);
        let draw_data = self.imgui.render();

        self
            .renderer
            .render(target, draw_data)
            .expect("imgui rendering failed");
    }

    pub fn ui<F>(&mut self, build: F)
    where
        F: FnOnce(&Ui),
    {
        let ui = self.imgui.frame();
        build(&ui);
    }
}
