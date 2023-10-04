use env_logger::Env;

///Logger is used to log to console
pub(crate) struct Logger;

///Use this version for logging info
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => (info!($($arg)*));
}
///Use this version for logging errors
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => (error!($($arg)*));
}
///Use this version for logging warnings
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => (warn!($($arg)*));
}
///Use this version for logging debug
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => (debug!($($arg)*));
}

impl Logger{
///Initalizes the logger
///Must be called first before calling any log macros
    pub fn init(){
        env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
    }
}