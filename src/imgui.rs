use std::{rc::Rc, time::Instant};

use winit::event::{WindowEvent, Event};

pub(crate) struct Imgui {
    window: Rc<winit::window::Window>,
    // device: &'a wgpu::Device,
    // queue: &'a wgpu::Queue,
    imgui_context: imgui::Context,
    imgui_platform: imgui_winit_support::WinitPlatform, 
    imgui_renderer: imgui_wgpu::Renderer,
    last_cursor: Option<imgui::MouseCursor>,
    last_frame: Instant,
}

impl Imgui {
    pub(crate) fn new(window: Rc<winit::window::Window>, device: &wgpu::Device, queue: &wgpu::Queue,
        surface_format: &wgpu::SurfaceConfiguration) -> Self {
        let mut imgui_context = imgui::Context::create();
        let mut imgui_platform = imgui_winit_support::WinitPlatform::init(&mut imgui_context);
        imgui_platform.attach_window(imgui_context.io_mut(), &window, imgui_winit_support::HiDpiMode::Default);
        
        let hidpi_factor = window.scale_factor();
        let font_size = (13.0 * hidpi_factor) as f32;
        imgui_context.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
        imgui_context.fonts().add_font(&[imgui::FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);

        let renderer_config = imgui_wgpu::RendererConfig {
            texture_format: surface_format.format,
            ..Default::default()
        };

        let imgui_renderer = imgui_wgpu::Renderer::new(&mut imgui_context, device, queue, renderer_config);
        let last_frame = Instant::now();

        Self {
            window,
            imgui_context,
            imgui_platform,
            imgui_renderer,
            last_cursor: None,
            last_frame,
        }
    }

    pub(crate) fn render_ui<'a>(&'a mut self, device: &wgpu::Device, queue: &wgpu::Queue, pass: &mut wgpu::RenderPass<'a>) {
        // let delta_t = self.last_frame.elapsed();
        let now = Instant::now();
        self.imgui_context.io_mut().update_delta_time(now - self.last_frame);
        self.last_frame = now;
        self.imgui_platform.prepare_frame(self.imgui_context.io_mut(), &self.window).expect("failed to prepare frame");
        let ui = self.imgui_context.frame();
        {
            ui.show_demo_window(&mut true);
        }
        if self.last_cursor != ui.mouse_cursor() {
            self.last_cursor = ui.mouse_cursor();
            self.imgui_platform.prepare_render(&ui, &self.window);
        }
        
        self.imgui_renderer.render(ui.render(), queue, device,pass).expect("failed to render imgui window");
    }

    pub(crate) fn event(&mut self, event: &Event<()>) {
        self.imgui_platform.handle_event(self.imgui_context.io_mut(), &self.window, event);
    }
}