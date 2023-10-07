use sdl2;
use log::*;

use crate::logger::Logger;
use crate::events::{EventSystem, EventType};
use crate::imgui_wrapper;

/// Entry point of the game engine.
///
/// Use the [`create`] method to create an `Application` instance.
/// Then use the [`run`] method to run the game engine.
///
/// [`create`]: Application::create
/// [`run`]: Application::run
pub struct Application{
    _window: sdl2::video::Window,
    event_system: EventSystem,
    imgui: imgui_wrapper::ImguiRuntime
}

impl Application{

    pub fn create()->Application{
        Logger::init();

        log_warn!("window is created in application init fn: change later");
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        
        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_version(3, 3);
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);

        let window = video_subsystem.window("Rose Engine", 800, 800).opengl().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let event_system = EventSystem::new(event_pump);
        let imgui_runtime = imgui_wrapper::ImguiRuntime::new(&window);
        

        Application{
            _window: window,
            event_system,
            imgui:imgui_runtime,
        }
    }

    pub fn run(&mut self){
        log_info!("Running Rose Engine main loop");
        loop{
            self.event_system.update();
            if self.handle_events()==false{
                break;
            }
            self.imgui.update(&self.event_system, &self._window);
        }
    }

    fn handle_events(&mut self)->bool {
        for event in self.event_system.iter() {
            log_info!("{:?}1",event.event_type);
            if event.event_type==EventType::ExitEvent{
                return false;
            }
        }
        return true;
    }
}