use log::*;
pub mod logger;
use logger::Logger;

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