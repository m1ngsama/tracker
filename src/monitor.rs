use crate::config::Config;
use crate::process::ProcessMonitor;
use crate::temperature::TemperatureMonitor;
use crate::alert::AlertSystem;
use crate::logger::TrackerLogger;
use sysinfo::{System, Disks, Networks};
use chrono::Local;
use crossterm::style::Stylize;

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

    fn draw_bar(&self, percent: f32) -> String {
        let width = 20;
        let filled = ((percent / 100.0) * width as f32).round() as usize;
        let filled = filled.min(width); // Ensure we don't exceed width
        let empty = width.saturating_sub(filled);
        
        let bar_str = format!("[{}{}]", "|".repeat(filled), " ".repeat(empty));
        
        if percent >= 90.0 {
            bar_str.red().to_string()
        } else if percent >= 75.0 {
            bar_str.yellow().to_string()
        } else {
            bar_str.green().to_string()
        }
    }

    pub fn get_cpu_usage(&mut self) -> f32 {
        self.sys.refresh_cpu_all();
        self.sys.global_cpu_usage()
    }

    pub fn get_memory_info(&mut self) -> MemoryInfo {
        self.sys.refresh_memory();
        let total = self.sys.total_memory();
        let used = self.sys.used_memory();
        let available = self.sys.available_memory();
        let percent = if total > 0 {
            (used as f32 / total as f32) * 100.0
        } else {
            0.0
        };

        MemoryInfo {
            total,
            used,
            available,
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
            free: available,
            percent,
        }
    }

    pub fn get_network_stats(&mut self) -> NetworkStats {
        self.networks.refresh();
        
        let mut bytes_sent = 0;
        let mut bytes_recv = 0;
        let mut packets_sent = 0;
        let mut packets_recv = 0;
        
        for (_, network) in &self.networks {
            bytes_sent += network.total_transmitted();
            bytes_recv += network.total_received();
            packets_sent += network.total_packets_transmitted();
            packets_recv += network.total_packets_received();
        }

        NetworkStats {
            bytes_sent,
            bytes_recv,
            packets_sent,
            packets_recv,
        }
    }

    pub fn display_stats(&mut self) {
        println!("\n{}", "=".repeat(50).blue());
        println!("System Tracker - {}", Local::now().format("%Y-%m-%d %H:%M:%S").to_string().cyan().bold());
        println!("{}\n", "=".repeat(50).blue());

        if self.config.display.show_cpu {
            let cpu_usage = self.get_cpu_usage();
            let bar = self.draw_bar(cpu_usage);
            println!("{:<15} {} {:.2}%", "CPU Usage:".bold(), bar, cpu_usage);
            self.logger.log_stats("CPU", &format!("{:.2}%", cpu_usage));
            self.alert_system.check_cpu_alert(cpu_usage);
        }

        if self.config.display.show_memory {
            let mem = self.get_memory_info();
            let bar = self.draw_bar(mem.percent);
            println!("{:<15} {} {:.2}% ({:.2}GB / {:.2}GB)", 
                "Memory:".bold(),
                bar,
                mem.percent,
                mem.used as f64 / (1024_f64.powi(3)),
                mem.total as f64 / (1024_f64.powi(3))
            );
            self.logger.log_stats("Memory", &format!("{:.2}%", mem.percent));
            self.alert_system.check_memory_alert(mem.percent);
        }

        if self.config.display.show_disk {
            let disk = self.get_disk_usage();
            let bar = self.draw_bar(disk.percent);
            println!("{:<15} {} {:.2}% ({:.2}GB / {:.2}GB)", 
                "Disk:".bold(),
                bar,
                disk.percent,
                disk.used as f64 / (1024_f64.powi(3)),
                disk.total as f64 / (1024_f64.powi(3))
            );
            self.logger.log_stats("Disk", &format!("{:.2}%", disk.percent));
            self.alert_system.check_disk_alert(disk.percent);
        }

        if self.config.display.show_network {
            let net = self.get_network_stats();
            println!("{:<15} Sent {} | Recv {}", 
                "Network:".bold(),
                format!("{:.2}MB", net.bytes_sent as f64 / (1024_f64.powi(2))).green(),
                format!("{:.2}MB", net.bytes_recv as f64 / (1024_f64.powi(2))).green()
            );
            self.logger.log_stats("Network", &format!("Sent: {} Recv: {}", net.bytes_sent, net.bytes_recv));
        }

        if self.config.display.show_processes {
            println!("\n{}", "Top Processes:".bold().underlined());
            self.process_monitor.display_processes(self.config.process_limit);
        }

        if self.config.display.show_temperatures {
            println!("\n{}", "Temperatures:".bold().underlined());
            self.temperature_monitor.display_temperatures();
        }
    }
}

#[derive(Debug)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    #[allow(dead_code)]
    pub available: u64,
    pub percent: f32,
}

#[derive(Debug)]
pub struct DiskInfo {
    pub total: u64,
    pub used: u64,
    #[allow(dead_code)]
    pub free: u64,
    pub percent: f32,
}

#[derive(Debug)]
pub struct NetworkStats {
    pub bytes_sent: u64,
    pub bytes_recv: u64,
    #[allow(dead_code)]
    pub packets_sent: u64,
    #[allow(dead_code)]
    pub packets_recv: u64,
}
