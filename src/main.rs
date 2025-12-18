use clap::Parser;
use std::thread;
use std::time::Duration;

mod config;
mod monitor;
mod process;
mod temperature;
mod alert;
mod logger;

use config::Config;
use monitor::SystemMonitor;

#[derive(Parser, Debug)]
#[command(name = "tracker-rs")]
#[command(about = "System Tracker - Monitor machine health and performance", long_about = None)]
struct Args {
    /// Run in continuous monitoring mode
    #[arg(short, long)]
    continuous: bool,

    /// Update interval in seconds
    #[arg(short, long, default_value_t = 5)]
    interval: u64,

    /// Path to config file
    #[arg(long, default_value = "config.json")]
    config: String,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    let config = Config::load(&args.config).unwrap_or_else(|_| {
        log::warn!("Failed to load config, using defaults");
        Config::default()
    });

    let mut monitor = SystemMonitor::new(config);

    if args.continuous {
        log::info!("Starting continuous monitoring mode with {}s interval", args.interval);
        loop {
            monitor.display_stats();
            thread::sleep(Duration::from_secs(args.interval));
        }
    } else {
        monitor.display_stats();
    }
}
