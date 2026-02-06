use tracing_appender::rolling::RollingFileAppender;
use tracing_subscriber::{
    fmt,
    prelude::*,
    layer::{Layer, Filter},
};
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU8, Ordering};
use std::io::{Write, Result};
use tracing::{Metadata, Level, Subscriber};

pub struct SwapWriter {
    inner: Arc<Mutex<NonBlocking>>,
}

impl Write for SwapWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.inner.lock().unwrap().write(buf)
    }
    fn flush(&mut self) -> Result<()> {
        self.inner.lock().unwrap().flush()
    }
}

impl<'a> fmt::MakeWriter<'a> for SwapWriter {
    type Writer = Self;
    fn make_writer(&'a self) -> Self::Writer {
        Self { inner: self.inner.clone() }
    }
}

// Custom dynamic log level filter
pub struct DynamicLevelFilter {
    current_level: Arc<AtomicU8>,
}

impl DynamicLevelFilter {
    fn get_level(&self) -> Level {
        map_level(self.current_level.load(Ordering::Relaxed))
    }
}

impl<S: Subscriber> Filter<S> for DynamicLevelFilter {
    fn enabled(&self, metadata: &Metadata<'_>, _ctx: &tracing_subscriber::layer::Context<'_, S>) -> bool {
        metadata.level() <= &self.get_level()
    }
}



pub struct LoggingSystem {
    pub writer: SwapWriter,
    pub guard: Arc<Mutex<WorkerGuard>>,
    pub level: Arc<AtomicU8>,
    pub log_dir: Arc<Mutex<String>>,
    pub timezone_offset: FixedOffset,
}

use tracing_subscriber::fmt::time::FormatTime;
use chrono::FixedOffset;

#[derive(Clone, Copy)]
struct LocalTimer {
    offset: FixedOffset,
}

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut fmt::format::Writer<'_>) -> std::fmt::Result {
        let now = chrono::Utc::now().with_timezone(&self.offset);
        write!(w, "{}", now.format("%Y-%m-%dT%H:%M:%S%.6f%:z"))
    }
}

fn parse_timezone(tz_str: &str) -> FixedOffset {
    // Expected format from settings: "UTC+08:00"
    let clean_tz = tz_str.trim_start_matches("UTC");
    if clean_tz.is_empty() {
        return FixedOffset::east_opt(0).unwrap();
    }

    // Attempt to parse offset part like "+08:00" or "+0800"
    match chrono::DateTime::parse_from_rfc3339(&format!("2024-01-01T00:00:00{}", clean_tz)) {
        Ok(dt) => *dt.offset(),
        Err(_) => {
            // Fallback: search for sign and try parsing hours
            FixedOffset::east_opt(0).unwrap()
        }
    }
}

pub fn init_logging(log_dir: &str, level_num: u8, timezone_str: &str) -> LoggingSystem {
    let level_atomic = Arc::new(AtomicU8::new(level_num));
    let offset = parse_timezone(timezone_str);
    let timer = LocalTimer { offset };
    
    let log_dir_arc = Arc::new(Mutex::new(log_dir.to_string()));

    // Use current local date for the initial filename
    let now = chrono::Utc::now().with_timezone(&offset);
    let date_str = now.format("%Y-%m-%d").to_string();
    let filename_prefix = format!("wsl-dashboard.{}", date_str);
    
    // Terminal output
    let stdout_layer = fmt::layer()
        .with_timer(timer)
        .with_target(false)
        .with_filter(DynamicLevelFilter { current_level: level_atomic.clone() });
    
    // File output with manual rotation (Rotation::NEVER)
    // We handle rotation manually to respect local timezone (daily at 00:00 Local)
    let file_appender = RollingFileAppender::builder()
        .rotation(tracing_appender::rolling::Rotation::NEVER) 
        .filename_prefix(&filename_prefix)
        .filename_suffix("log")
        .build(log_dir)
        .expect("Failed to create log appender");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    
    let swap_writer = SwapWriter { inner: Arc::new(Mutex::new(non_blocking)) };
    let guard_holder = Arc::new(Mutex::new(guard));

    let file_layer = fmt::layer()
        .with_timer(timer)
        .with_writer(swap_writer.clone())
        .with_ansi(false)
        .with_target(true)
        .with_filter(DynamicLevelFilter { current_level: level_atomic.clone() });

    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(file_layer)
        .init();

    // Spawn rotation thread
    spawn_rotation_thread(
        swap_writer.clone(), 
        guard_holder.clone(), 
        log_dir_arc.clone(), 
        offset
    );

    LoggingSystem { 
        writer: swap_writer, 
        guard: guard_holder,
        level: level_atomic,
        log_dir: log_dir_arc,
        timezone_offset: offset,
    }
}

fn spawn_rotation_thread(
    writer: SwapWriter, 
    guard: Arc<Mutex<WorkerGuard>>, 
    log_dir: Arc<Mutex<String>>, 
    offset: FixedOffset
) {
    std::thread::spawn(move || {
        let mut last_date = chrono::Utc::now().with_timezone(&offset).date_naive();

        loop {
            // Check every 30 seconds for date change
            // This handles both natural time progression and manual system time changes
            std::thread::sleep(std::time::Duration::from_secs(30));

            let now = chrono::Utc::now().with_timezone(&offset);
            let current_date = now.date_naive();

            if current_date > last_date {
                // Date has changed (forward), perform rotation
                if let Ok(current_dir) = log_dir.lock() {
                    let date_str = now.format("%Y-%m-%d").to_string();
                    let filename_prefix = format!("wsl-dashboard.{}", date_str);
                    
                    let file_appender = RollingFileAppender::builder()
                        .rotation(tracing_appender::rolling::Rotation::NEVER)
                        .filename_prefix(&filename_prefix)
                        .filename_suffix("log")
                        .build(current_dir.as_str());
                        
                    match file_appender {
                        Ok(appender) => {
                             let (non_blocking, new_guard) = tracing_appender::non_blocking(appender);
                             
                             // Atomically swap the writer and guard
                             if let Ok(mut w) = writer.inner.lock() {
                                 *w = non_blocking;
                             }
                             if let Ok(mut g) = guard.lock() {
                                 *g = new_guard;
                             }
                             
                             // Update last_known date
                             last_date = current_date;
                        },
                        Err(e) => {
                            eprintln!("[Rotation] Failed to rotate log: {}", e);
                            // Do not update last_date, will try again next cycle
                        }
                    }
                }
            } else if current_date < last_date {
                // Time jump backwards? Just update tracking date so we don't rotate unnecessarily
                // until we hit the 'next' day relative to this new time.
                last_date = current_date;
            }
        }
    });
}

fn map_level(level_num: u8) -> Level {
    match level_num {
        1 => Level::ERROR,
        2 => Level::WARN,
        3 => Level::INFO,
        4 => Level::DEBUG,
        5 => Level::TRACE,
        _ => Level::INFO,
    }
}

impl LoggingSystem {
    pub fn update_path(&self, log_dir: &str) {
        // Update stored path for rotation thread
        if let Ok(mut d) = self.log_dir.lock() {
            *d = log_dir.to_string();
        }

        // Re-create appender immediately
        let now = chrono::Utc::now().with_timezone(&self.timezone_offset);
        let date_str = now.format("%Y-%m-%d").to_string();
        let filename_prefix = format!("wsl-dashboard.{}", date_str);

        let file_appender = RollingFileAppender::builder()
            .rotation(tracing_appender::rolling::Rotation::NEVER)
            .filename_prefix(&filename_prefix)
            .filename_suffix("log")
            .build(log_dir)
            .expect("Failed to create log appender");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        
        *self.writer.inner.lock().unwrap() = non_blocking;
        *self.guard.lock().unwrap() = guard;
        tracing::info!("Log directory updated to: {}", log_dir);
    }

    pub fn update_level(&self, level_num: u8) {
        self.level.store(level_num, Ordering::Relaxed);
        let level = map_level(level_num);
        tracing::info!("Log level updated to: {:?}", level);
    }
}

impl Clone for SwapWriter {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

// Cleanup expired log files
pub fn cleanup_expired_logs(log_dir: &str, log_days: u8) {
    let log_path = std::path::Path::new(log_dir);
    if !log_path.exists() || !log_path.is_dir() {
        return;
    }

    let now = chrono::Local::now().date_naive();
    let expiration_date = now - chrono::TimeDelta::days(log_days as i64);

    if let Ok(entries) = std::fs::read_dir(log_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                    // Pattern: wsl-dashboard.YYYY-MM-DD.log
                    if file_name.starts_with("wsl-dashboard.") && file_name.ends_with(".log") {
                        let date_part = file_name
                            .trim_start_matches("wsl-dashboard.")
                            .trim_end_matches(".log");
                        
                        if let Ok(file_date) = chrono::NaiveDate::parse_from_str(date_part, "%Y-%m-%d") {
                            if file_date <= expiration_date {
                                tracing::info!("Deleting expired log file: {:?}", file_name);
                                let _ = std::fs::remove_file(path);
                            }
                        }
                    }
                }
            }
        }
    }
}
