use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub update_interval: u64,
    pub display: DisplayConfig,
    pub process_limit: usize,
    pub alert_thresholds: AlertThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub show_cpu: bool,
    pub show_memory: bool,
    pub show_disk: bool,
    pub show_network: bool,
    pub show_processes: bool,
    pub show_temperatures: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub cpu_percent: f32,
    pub memory_percent: f32,
    pub disk_percent: f32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            update_interval: 5,
            display: DisplayConfig {
                show_cpu: true,
                show_memory: true,
                show_disk: true,
                show_network: true,
                show_processes: true,
                show_temperatures: true,
            },
            process_limit: 5,
            alert_thresholds: AlertThresholds {
                cpu_percent: 80.0,
                memory_percent: 85.0,
                disk_percent: 90.0,
            },
        }
    }
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }
}
