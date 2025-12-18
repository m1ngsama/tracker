use crate::config::Config;
use crate::logger::TrackerLogger;

pub struct AlertSystem {
    config: Config,
    logger: TrackerLogger,
}

impl AlertSystem {
    pub fn new(config: Config) -> Self {
        AlertSystem {
            config,
            logger: TrackerLogger::default(),
        }
    }

    pub fn check_cpu_alert(&mut self, cpu_percent: f32) -> bool {
        let threshold = self.config.alert_thresholds.cpu_percent;
        if cpu_percent > threshold {
            let message = format!("CPU usage is {:.2}% (threshold: {:.2}%)", cpu_percent, threshold);
            self.trigger_alert("CPU", &message);
            return true;
        }
        false
    }

    pub fn check_memory_alert(&mut self, memory_percent: f32) -> bool {
        let threshold = self.config.alert_thresholds.memory_percent;
        if memory_percent > threshold {
            let message = format!("Memory usage is {:.2}% (threshold: {:.2}%)", memory_percent, threshold);
            self.trigger_alert("Memory", &message);
            return true;
        }
        false
    }

    pub fn check_disk_alert(&mut self, disk_percent: f32) -> bool {
        let threshold = self.config.alert_thresholds.disk_percent;
        if disk_percent > threshold {
            let message = format!("Disk usage is {:.2}% (threshold: {:.2}%)", disk_percent, threshold);
            self.trigger_alert("Disk", &message);
            return true;
        }
        false
    }

    fn trigger_alert(&mut self, alert_type: &str, message: &str) {
        self.logger.log_alert(&format!("{}: {}", alert_type, message));
        println!("\n⚠️  ALERT: {}", message);
    }
}