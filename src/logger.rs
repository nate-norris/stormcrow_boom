//! Starts async file-based log service for a headless program.
//! 
//! The global 'tracing' subscriber writes log messages to a "log.txt" file 
//! co-located with the running executable. A background WorkerGuard handles
//! I/O so logging does not block the main program. The file is created automatically 
//! if missing.  
use std::env;
use std::path::PathBuf;
use std::sync::Once;
use std::fs;
use std::fs::OpenOptions;

use tracing::Level;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::fmt::time::UtcTime;

// ensure the logger is initalized once
static INIT: Once = Once::new();

// Non blocking logging gaurd. Initialized once and never mutated.
//
// Keeping this alive ensures logging threads remain active for the life of
//      the program.
static mut LOG_GUARD: Option<WorkerGuard> = None;

/// Starts async file-based log service for a headless program.
///
/// The global 'tracing' subscriber writes log messages to a "log.txt" file 
/// co-located with the running executable. A background WorkerGuard handles
/// I/O so logging does not block the main program. The file is created automatically 
/// if missing. 
/// 
/// # Usage
/// This should be called once at program startup, typically from `main()`:
///
/// ```no_run
/// logger::init_logger();
/// logger::info("Application started");
/// ```
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

        // open file
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)
            .unwrap();

        // create non-blocking writer and guard
        let (nb, g) = NonBlocking::new(file);
        unsafe { LOG_GUARD = Some(g); }

        // global tracing subscriber
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

/// Writes information logs
/// Arguments must implement ['std::fmt:;Display']
/// 
/// # Arguments
/// * `message` — The information to log
pub fn info(message: impl std::fmt::Display) {
    tracing::info!("{}", message);
}

/// Writes error logs
/// Arguments must implement ['std::fmt:;Display']
/// 
/// # Arguments
/// `message` - the primary error text
/// `extra` - an additional value to append to log
/// 
/// # Examples
/// ```
/// logger::error("sensor failure", None);
/// logger::error("sensor failure:", Some(err));
/// ```
pub fn error(message: impl std::fmt::Display, extra: Option<impl std::fmt::Display>) {
    match extra {
        Some(e) => tracing::error!("{} {}", message, e),
        None => tracing::error!("{}", message),
    }
}