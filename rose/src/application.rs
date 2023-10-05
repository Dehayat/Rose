use sdl2;
use log::*;

use crate::logger::Logger;

/// Entry point of the game engine.
///
/// Use the [`create`] method to create an `Application` instance.
/// Then use the [`run`] method to run the game engine.
///
/// [`create`]: Application::create
/// [`run`]: Application::run
pub struct Application{
    //sdl_context: sdl2::Sdl,
    event_pump: sdl2::EventPump,
    _window: sdl2::video::Window,
    //video_subsystem: sdl2::VideoSubsystem
}

impl Application{

    pub fn create()->Application{
        Logger::init();

        log_warn!("window is created in application init fn: change later");
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Rose Engine", 200, 200).opengl().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();


        Application{
            //sdl_context,
            event_pump,
            _window: window,
            //video_subsystem
        }
    }

    pub fn run(&mut self){
        log_info!("Running Rose Engine main loop");
        'main: loop{
            for event in self.event_pump.poll_iter() {
                use sdl2::event::Event;
                match event {
                    Event::Quit { .. } => break 'main,
                    _ => {}
                }
            }
        }
    }
}