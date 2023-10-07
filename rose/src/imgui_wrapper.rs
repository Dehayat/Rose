use std::time::Instant;

use glow::HasContext;
use imgui::Context;
use imgui;
use sdl2::video::Window;
use imgui_sdl2_support::SdlPlatform;
use imgui_glow_renderer::AutoRenderer;

use crate::events::EventSystem;



fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

pub struct ImguiRuntime {
    platform: SdlPlatform,
    imgui_context:Context,
    last_frame: Instant,
    renderer: AutoRenderer,
    _gl_context: sdl2::video::GLContext,
}

impl ImguiRuntime {
    pub fn new(window: &Window) -> Self {
        
        let gl_context: sdl2::video::GLContext = window.gl_create_context().unwrap();
        window.gl_make_current(&gl_context).unwrap();
        window.subsystem().gl_set_swap_interval(1).unwrap();

        let gl = glow_context(&window);

        let mut imgui = Context::create();
        
        imgui.set_ini_filename(None);
        imgui.set_log_filename(None);
        imgui
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

        let platform = SdlPlatform::init(&mut imgui);
        let renderer = AutoRenderer::initialize(gl, &mut imgui).unwrap();
        ImguiRuntime {platform,imgui_context:imgui,last_frame: Instant::now(),renderer,_gl_context:gl_context}
        
    }
    pub fn update(&mut self,event_system: &EventSystem,window: &Window) {
        for event in event_system.iter() {
            self.platform.handle_event(&mut self.imgui_context,&event.sdl_event);
        }
        self.prepare_frame(window,event_system);
        let ui = self.imgui_context.new_frame();
        ui.show_demo_window(&mut true);

        let draw_data = self.imgui_context.render();

        unsafe { self.renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
        self.renderer.render(draw_data).unwrap();
        window.gl_swap_window();
    }

    fn prepare_frame(&mut self,window: &Window,event_system: &EventSystem){
        let io = self.imgui_context.io_mut();

        // Update delta time
        let now = std::time::Instant::now();
        io.update_delta_time(now.duration_since(self.last_frame));
        self.last_frame = now;

        let mouse_state = event_system.get_mouse_state();

        io.mouse_pos = [mouse_state.x() as f32, mouse_state.y() as f32];

        let window_size = window.size();
        let window_drawable_size = window.drawable_size();

        io.display_size = [window_size.0 as f32, window_size.1 as f32];
        io.display_framebuffer_scale = [
            (window_drawable_size.0 as f32) / (window_size.0 as f32),
            (window_drawable_size.1 as f32) / (window_size.1 as f32),
        ];
    }
}
