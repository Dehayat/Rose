pub use log::*;
pub mod logger;
use logger::Logger;

/// Entry point of the game engine.
///
/// Use the [`create`] method to create an `Application` instance.
/// Then use the [`run`] method to run the game engine.
///
/// [`create`]: Application::create
/// [`run`]: Application::run
pub struct Application;

impl Application{

    pub fn create()->Application{
        Logger::init();
        Application{}
    }

    pub fn run(&self){
        log_info!("Running Rose Engine");
        // while true{
        // }
    }
}