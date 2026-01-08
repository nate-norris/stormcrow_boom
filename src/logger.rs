// logger.rs
use std::env;
use std::path::PathBuf;
use std::sync::Once;
use std::fs;
use std::fs::OpenOptions;

use tracing::Level;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::time::UtcTime;

static INIT: Once = Once::new();
static mut LOG_GUARD: Option<WorkerGuard> = None;

#[allow(dead_code)]
pub fn init_logger_old() {
    INIT.call_once(|| {
        // Determine current executable directory
        let exe_dir = env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(PathBuf::from))
            .unwrap_or_else(|| env::current_dir().unwrap());

        // Ensure log file exists
        let log_path = exe_dir.join("log.txt");
        if !log_path.exists() {
            fs::File::create(&log_path).unwrap();
        }

        // Use rolling appender with no rotation
        let file_appender: RollingFileAppender = RollingFileAppender::new(
            Rotation::NEVER, 
            exe_dir, 
            "log.txt");
        // gaurd ignored as background stays alive as long as the subscriber.
        let (_non_blocking, _guard) = 
            NonBlocking::new(file_appender);


        tracing_subscriber::fmt()
            .with_writer(std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_path.clone())
                .unwrap())
            // .with_writer(non_blocking)
            .compact() // single line
            .with_timer(UtcTime::rfc_3339())
            .with_max_level(Level::INFO)
            .with_level(true)
            .with_target(false)
            .with_ansi(false) // no color code
            .init();
    });
}

pub fn init_logger() {
    INIT.call_once( || {

        // path to current executable file
        let exe_dir = env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(PathBuf::from))
            .unwrap_or_else(|| env::current_dir().unwrap());

        let log_path = exe_dir.join("log.txt");
        //create the file if needed
        if !log_path.exists() {
            fs::File::create(&log_path).unwrap();
        }

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)
            .unwrap();

        let (nb, g) = NonBlocking::new(file);
        unsafe { LOG_GUARD = Some(g); }

        tracing_subscriber::fmt()
            .with_writer(nb)
            .compact() // single line
            .with_timer(UtcTime::rfc_3339())
            .with_max_level(Level::INFO)
            .with_level(true)
            .with_target(false)
            .with_ansi(false) // no color code
            .init();


    });
}

pub fn info(message: &str) {
    tracing::info!("{}", message);
}

pub fn error(message: &str) {
    tracing::error!("{}", message);
}