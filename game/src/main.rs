use rose::*;
use rose::application::Application;

fn main() {
    let mut app =  Application::create();
    log_info!("Starting Game");
    app.run();
}
