use sysinfo::System;
use crossterm::style::Stylize;

pub struct ProcessMonitor {
    sys: System,
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f32,
    pub memory_percent: f32,
}

impl ProcessMonitor {
    pub fn new() -> Self {
        ProcessMonitor {
            sys: System::new_all(),
        }
    }

    pub fn get_top_processes(&mut self, limit: usize) -> Vec<ProcessInfo> {
        self.sys.refresh_all();
        
        let mut processes: Vec<ProcessInfo> = self.sys.processes()
            .iter()
            .map(|(pid, process)| {
                let total_memory = self.sys.total_memory() as f32;
                let memory_percent = if total_memory > 0.0 {
                    (process.memory() as f32 / total_memory) * 100.0
                } else {
                    0.0
                };

                ProcessInfo {
                    pid: pid.as_u32(),
                    name: process.name().to_string_lossy().to_string(),
                    cpu_percent: process.cpu_usage(),
                    memory_percent,
                }
            })
            .collect();

        processes.sort_by(|a, b| b.cpu_percent.partial_cmp(&a.cpu_percent).unwrap());
        processes.truncate(limit);
        processes
    }

    pub fn get_process_count(&mut self) -> usize {
        self.sys.refresh_all();
        self.sys.processes().len()
    }

    pub fn display_processes(&mut self, limit: usize) {
        println!("{:<10}{:<30}{:<10}{:<10}", 
            "PID".bold(), "Name".bold(), "CPU%".bold(), "Memory%".bold());
        println!("{}", "-".repeat(60).blue());

        for proc in self.get_top_processes(limit) {
            let cpu = format!("{:.2}", proc.cpu_percent);
            let mem = format!("{:.2}", proc.memory_percent);
            
            let cpu_colored = if proc.cpu_percent > 50.0 {
                cpu.red()
            } else if proc.cpu_percent > 20.0 {
                cpu.yellow()
            } else {
                cpu.green()
            };

            let name = if proc.name.len() > 28 {
                format!("{}...", &proc.name[..25])
            } else {
                proc.name.clone()
            };

            println!("{:<10}{:<30}{:<10}{:<10}", 
                proc.pid, 
                name, 
                cpu_colored, 
                mem
            );
        }

        println!("\nTotal Processes: {}", self.get_process_count().to_string().cyan());
    }
}

impl Default for ProcessMonitor {
    fn default() -> Self {
        Self::new()
    }
}
