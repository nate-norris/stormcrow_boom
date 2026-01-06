// logger.rs
use std::env;
use std::fs;
use std::path::PathBuf;
use tracing::Level;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use std::sync::Once;

static mut LOG_GUARD: Option<WorkerGuard> = None;
static INIT: Once = Once::new();

pub fn init_logger() {
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
        let file_appender: RollingFileAppender = RollingFileAppender::new(Rotation::NEVER, exe_dir, "log.txt");
        let (non_blocking, guard) = NonBlocking::new(file_appender);

        unsafe { LOG_GUARD = Some(guard) }

        tracing_subscriber::fmt()
            .with_writer(non_blocking)
            .with_max_level(Level::INFO)
            .init();
    });
}

pub fn flush_logs() {
    unsafe {
        if let Some(guard) = &LOG_GUARD {
            guard.flush();
        }
    }
}