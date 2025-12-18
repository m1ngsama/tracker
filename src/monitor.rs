use crate::alert::AlertSystem;
use crate::config::Config;
use crate::logger::TrackerLogger;
use crate::process::ProcessMonitor;
use crate::temperature::TemperatureMonitor;
use chrono::Local;
use sysinfo::{Disks, Networks, System};

pub struct SystemMonitor {
    config: Config,
    sys: System,
    disks: Disks,
    networks: Networks,
    process_monitor: ProcessMonitor,
    temperature_monitor: TemperatureMonitor,
    alert_system: AlertSystem,
    logger: TrackerLogger,
}

impl SystemMonitor {
    pub fn new(config: Config) -> Self {
        SystemMonitor {
            config: config.clone(),
            sys: System::new_all(),
            disks: Disks::new_with_refreshed_list(),
            networks: Networks::new_with_refreshed_list(),
            process_monitor: ProcessMonitor::new(),
            temperature_monitor: TemperatureMonitor::new(),
            alert_system: AlertSystem::new(config),
            logger: TrackerLogger::default(),
        }
    }

    pub fn get_cpu_usage(&mut self) -> f32 {
        self.sys.refresh_cpu_all();
        std::thread::sleep(std::time::Duration::from_millis(200));
        self.sys.refresh_cpu_all();
        self.sys.global_cpu_usage()
    }

    pub fn get_memory_info(&mut self) -> MemoryInfo {
        self.sys.refresh_memory();
        let total = self.sys.total_memory();
        let used = self.sys.used_memory();
        let percent = if total > 0 {
            (used as f32 / total as f32) * 100.0
        } else {
            0.0
        };

        MemoryInfo {
            total,
            used,
            percent,
        }
    }

    pub fn get_disk_usage(&mut self) -> DiskInfo {
        self.disks.refresh();

        let mut total: u64 = 0;
        let mut available: u64 = 0;

        for disk in &self.disks {
            total += disk.total_space();
            available += disk.available_space();
        }

        let used = total.saturating_sub(available);
        let percent = if total > 0 {
            (used as f32 / total as f32) * 100.0
        } else {
            0.0
        };

        DiskInfo {
            total,
            used,
            percent,
        }
    }

    pub fn get_network_stats(&mut self) -> NetworkStats {
        self.networks.refresh();

        let mut bytes_sent = 0;
        let mut bytes_recv = 0;

        for (_, network) in &self.networks {
            bytes_sent += network.total_transmitted();
            bytes_recv += network.total_received();
        }

        NetworkStats {
            bytes_sent,
            bytes_recv,
        }
    }

    pub fn display_stats(&mut self) {
        println!("\n{}", "=".repeat(50));
        println!(
            "System Tracker - {}",
            Local::now().format("%Y-%m-%d %H:%M:%S")
        );
        println!("{}\n", "=".repeat(50));

        if self.config.display.show_cpu {
            let cpu_usage = self.get_cpu_usage();
            println!("CPU Usage: {:.2}%", cpu_usage);
            self.logger.log_stats("CPU", &format!("{:.2}%", cpu_usage));
            self.alert_system.check_cpu_alert(cpu_usage);
        }

        if self.config.display.show_memory {
            let mem = self.get_memory_info();
            println!(
                "Memory: {:.2}% ({:.2}GB / {:.2}GB)",
                mem.percent,
                mem.used as f64 / (1024_f64.powi(3)),
                mem.total as f64 / (1024_f64.powi(3))
            );
            self.logger
                .log_stats("Memory", &format!("{:.2}%", mem.percent));
            self.alert_system.check_memory_alert(mem.percent);
        }

        if self.config.display.show_disk {
            let disk = self.get_disk_usage();
            println!(
                "Disk: {:.2}% ({:.2}GB / {:.2}GB)",
                disk.percent,
                disk.used as f64 / (1024_f64.powi(3)),
                disk.total as f64 / (1024_f64.powi(3))
            );
            self.logger
                .log_stats("Disk", &format!("{:.2}%", disk.percent));
            self.alert_system.check_disk_alert(disk.percent);
        }

        if self.config.display.show_network {
            let net = self.get_network_stats();
            println!(
                "Network: Sent {:.2}MB | Recv {:.2}MB",
                net.bytes_sent as f64 / (1024_f64.powi(2)),
                net.bytes_recv as f64 / (1024_f64.powi(2))
            );
            self.logger.log_stats(
                "Network",
                &format!("Sent: {} Recv: {}", net.bytes_sent, net.bytes_recv),
            );
        }

        if self.config.display.show_processes {
            self.process_monitor
                .display_processes(self.config.process_limit);
        }

        if self.config.display.show_temperatures {
            self.temperature_monitor.display_temperatures();
        }
    }
}

#[derive(Debug)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub percent: f32,
}

#[derive(Debug)]
pub struct DiskInfo {
    pub total: u64,
    pub used: u64,
    pub percent: f32,
}

#[derive(Debug)]
pub struct NetworkStats {
    pub bytes_sent: u64,
    pub bytes_recv: u64,
}
