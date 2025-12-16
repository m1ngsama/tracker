use chrono::Local;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

pub struct TrackerLogger {
    log_dir: PathBuf,
}

impl TrackerLogger {
    pub fn new(log_dir: &str) -> Self {
        let log_dir = PathBuf::from(log_dir);
        if !log_dir.exists() {
            fs::create_dir_all(&log_dir).ok();
        }
        TrackerLogger { log_dir }
    }

    fn get_log_file(&self) -> PathBuf {
        let date = Local::now().format("%Y%m%d").to_string();
        self.log_dir.join(format!("tracker_{}.log", date))
    }

    fn write_log(&self, level: &str, message: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_message = format!("{} - SystemTracker - {} - {}\n", timestamp, level, message);

        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.get_log_file())
        {
            let _ = file.write_all(log_message.as_bytes());
        }
    }

    pub fn log_stats(&self, stats_type: &str, stats_data: &str) {
        let message = format!("{}: {}", stats_type, stats_data);
        self.write_log("INFO", &message);
    }

    pub fn log_alert(&self, message: &str) {
        let alert_message = format!("ALERT: {}", message);
        self.write_log("WARNING", &alert_message);
    }

    #[allow(dead_code)]
    pub fn log_error(&self, error_message: &str) {
        let error_msg = format!("ERROR: {}", error_message);
        self.write_log("ERROR", &error_msg);
    }
}

impl Default for TrackerLogger {
    fn default() -> Self {
        Self::new("logs")
    }
}
