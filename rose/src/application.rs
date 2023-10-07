use log::*;
use sdl2::{self, pixels::Color};

use crate::events::{EventSystem, EventType};
use crate::imgui_wrapper;
use crate::logger::Logger;

/// Entry point of the game engine.
///
/// Use the [`create`] method to create an `Application` instance.
/// Then use the [`run`] method to run the game engine.
///
/// [`create`]: Application::create
/// [`run`]: Application::run
pub struct Application {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_system: EventSystem,
    imgui: imgui_wrapper::ImguiRuntime,
}

impl Application {
    pub fn create() -> Application {
        Logger::init();

        log_warn!("window is created in application init fn: change later");
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_version(3, 3);
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);

        let event_pump = sdl_context.event_pump().unwrap();
        let event_system = EventSystem::new(event_pump);

        let imgui_window = video_subsystem
            .window("Rose Engine Imgui", 800, 800)
            .opengl()
            .build()
            .unwrap();
        let imgui_runtime = imgui_wrapper::ImguiRuntime::new(imgui_window);

        let sdl_window = video_subsystem
            .window("Rose Engine", 400, 400)
            .build()
            .unwrap();
        let mut canvas = sdl_window.into_canvas().present_vsync().build().unwrap();

        canvas.set_draw_color(Color::RGB(20, 20, 20));

        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 210, 0));
        canvas.fill_rect(sdl2::rect::Rect::new(10, 10, 100, 200)).unwrap();

        canvas.present();

        Application {
            canvas,
            event_system,
            imgui: imgui_runtime,
        }
    }

    pub fn run(&mut self) {
        log_info!("Running Rose Engine main loop");
        loop {
            self.event_system.update();
            if self.handle_events() == false {
                break;
            }
            self.imgui.update(&self.event_system);
        }
    }

    fn handle_events(&mut self) -> bool {
        for event in self.event_system.iter() {
            if event.sdl_event.is_window()
                && event.sdl_event.get_window_id().unwrap() != self.canvas.window().id()
            {
                continue;
            }
            log_info!("{:?}", event.event_type);
            if event.event_type == EventType::ExitEvent {
                return false;
            }
        }
        return true;
    }
}
