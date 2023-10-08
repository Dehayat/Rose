use log::*;
use sdl2;

use crate::events::{EventSystem, EventType};
use crate::imgui_wrapper;
use crate::logger::Logger;
use crate::renderer;

/// Entry point of the game engine.
///
/// Use the [`create`] method to create an `Application` instance.
/// Then use the [`run`] method to run the game engine.
///
/// [`create`]: Application::create
/// [`run`]: Application::run
pub struct Application {
    renderer: renderer::Renderer,
    event_system: EventSystem,
    imgui: imgui_wrapper::ImguiRuntime,
    window_id:u32,
}

impl Application {
    pub fn new() -> Application{
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
        let window_id = sdl_window.id();
        Application {
            renderer: renderer::Renderer::new(sdl_window),
            event_system,
            imgui: imgui_runtime,
            window_id,
        }
    }

    pub fn run(&mut self) {
        log_info!("Running Rose Engine main loop");
        loop {
            self.event_system.update();
            if self.handle_events() == false {
                break;
            }
            self.renderer.render();

            
            self.imgui.update(&self.event_system);
            std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    fn handle_events(&mut self) -> bool {
        for event in self.event_system.iter() {
            if event.sdl_event.is_window()
                && event.sdl_event.get_window_id().unwrap() != self.window_id
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
