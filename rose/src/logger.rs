use env_logger::Env;

pub(crate) struct Logger;

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => (info!($($arg)*));
}
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => (error!($($arg)*));
}
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => (warn!($($arg)*));
}
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => (debug!($($arg)*));
}

impl Logger{

    pub fn init(){
        env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
    }
}