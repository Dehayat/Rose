use log::*;
use rose::*;

fn main() {
    let app =  rose::Application::create();
    log_info!("Starting Game");
    app.run();
}
