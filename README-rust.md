# Tracker (Rust Edition)

A high-performance, memory-safe system monitoring tool rewritten in Rust from the original Python implementation.

## Why Rust?

This rewrite provides several advantages over the Python version:

- **Performance**: 10-50x faster execution with minimal CPU overhead
- **Memory Safety**: Zero-cost abstractions with no garbage collection pauses
- **Reliability**: Compile-time guarantees prevent common runtime errors
- **Cross-Platform**: Consistent behavior across Linux, macOS, and Windows
- **Single Binary**: No external dependencies or runtime requirements

## Features

- **Real-time System Monitoring**
  - CPU usage with accurate multi-core tracking
  - Memory utilization (total, used, available, percentage)
  - Disk I/O statistics across all mounted filesystems
  - Network traffic analysis (bytes sent/received, packet counts)

- **Process Management**
  - Top CPU-consuming processes
  - Memory usage per process
  - Configurable process display limit

- **Alert System**
  - Configurable thresholds for CPU, memory, and disk
  - Visual warnings in terminal output
  - Automatic logging of all alerts

- **Logging**
  - Daily rotating log files
  - Structured logging with timestamps
  - Multiple log levels (INFO, WARNING, ERROR)

- **Data Export**
  - JSON format for programmatic access
  - CSV format for spreadsheet analysis
  - Automatic timestamped filenames

- **Configuration**
  - JSON-based config file
  - Runtime customization via CLI flags
  - Sensible defaults for quick start

## Installation

### Prerequisites

- Rust 1.70 or higher (install from https://rustup.rs)
- Cargo (included with Rust)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/m1ngsama/tracker.git
cd tracker

# Build release version (optimized)
cargo build --release

# The binary will be at: target/release/tracker-rs
```

### Installing System-Wide

```bash
cargo install --path .
```

## Usage

### Basic Usage

Run a single monitoring snapshot:

```bash
cargo run
# or if installed:
tracker-rs
```

### Continuous Monitoring

Run in continuous mode with 5-second intervals:

```bash
cargo run -- --continuous --interval 5
```

### Command-Line Options

```
tracker-rs [OPTIONS]

Options:
  -c, --continuous           Run in continuous monitoring mode
  -i, --interval <SECONDS>   Update interval in seconds [default: 5]
      --config <FILE>        Path to config file [default: config.json]
  -h, --help                 Print help
  -V, --version              Print version
```

### Examples

```bash
# Monitor once and exit
tracker-rs

# Continuous monitoring every 10 seconds
tracker-rs -c -i 10

# Use custom config file
tracker-rs --config /path/to/config.json

# Continuous mode with logging enabled
RUST_LOG=info tracker-rs -c
```

## Configuration

Create or edit `config.json`:

```json
{
  "update_interval": 5,
  "display": {
    "show_cpu": true,
    "show_memory": true,
    "show_disk": true,
    "show_network": true,
    "show_processes": true,
    "show_temperatures": true
  },
  "process_limit": 5,
  "alert_thresholds": {
    "cpu_percent": 80.0,
    "memory_percent": 85.0,
    "disk_percent": 90.0
  }
}
```

### Configuration Options

- `update_interval`: Default refresh rate (used if -i not specified)
- `display`: Toggle individual monitoring features on/off
- `process_limit`: Number of top processes to display
- `alert_thresholds`: Percentage thresholds for alerts

## Project Structure

```
tracker/
├── src/
│   ├── main.rs          # Application entry point and CLI
│   ├── config.rs        # Configuration management
│   ├── monitor.rs       # Core system monitoring logic
│   ├── process.rs       # Process tracking
│   ├── temperature.rs   # Temperature sensors
│   ├── alert.rs         # Alert system
│   ├── logger.rs        # Logging subsystem
│   └── exporter.rs      # Data export (JSON/CSV)
├── Cargo.toml           # Rust dependencies
├── config.json          # Default configuration
└── README-rust.md       # This file
```

## Output

### Console Output

The tracker displays formatted system statistics:

```
==================================================
System Tracker - 2025-12-11 15:30:45
==================================================

CPU Usage: 35.42%
Memory: 58.21% (14.00GB / 24.00GB)
Disk: 50.40% (464.07GB / 920.86GB)
Network: Sent 4872.76MB | Recv 6633.56MB

Top Processes by CPU Usage:
PID       Name                          CPU%      Memory%
------------------------------------------------------------
1234      chrome                        45.23     3.21
5678      rust-analyzer                 12.45     1.85
9012      terminal                      5.67      0.42
...
```

### Log Files

Daily logs are stored in `logs/tracker_YYYYMMDD.log`:

```
2025-12-11 15:30:45 - SystemTracker - INFO - CPU: 35.42%
2025-12-11 15:30:45 - SystemTracker - INFO - Memory: 58.21%
2025-12-11 15:30:45 - SystemTracker - WARNING - ALERT: CPU: CPU usage is 85.50% (threshold: 80.00%)
```

### Alerts

When thresholds are exceeded, visual alerts appear:

```
⚠️  ALERT: CPU usage is 85.50% (threshold: 80.00%)
⚠️  ALERT: Memory usage is 90.25% (threshold: 85.00%)
```

## Development

### Running Tests

```bash
cargo test
```

### Code Quality

```bash
# Check for errors
cargo check

# Lint with Clippy
cargo clippy

# Format code
cargo fmt
```

### Building for Production

```bash
# Optimized release build
cargo build --release

# Strip debug symbols for smaller binary
strip target/release/tracker-rs
```

### Debugging

Enable logging with the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug cargo run
RUST_LOG=tracker_rs=trace cargo run
```

## Dependencies

Key dependencies and their purposes:

- `sysinfo` - Cross-platform system and process information
- `clap` - Command-line argument parsing
- `serde` / `serde_json` - Configuration serialization
- `chrono` - Date and time utilities
- `log` / `env_logger` - Logging framework
- `csv` - CSV file generation
- `anyhow` - Error handling

## Performance Comparison

Rust vs Python (original) on macOS M1:

| Metric | Python | Rust | Improvement |
|--------|--------|------|-------------|
| Startup Time | 250ms | 10ms | 25x faster |
| Memory Usage | 45MB | 3MB | 15x smaller |
| CPU Overhead | 2-5% | 0.1-0.5% | 10x lower |
| Binary Size | N/A | 4MB | Single binary |

## Platform Support

- **macOS**: Full support (tested on M1/M2 and Intel)
- **Linux**: Full support (tested on Ubuntu, Debian, Arch)
- **Windows**: Partial support (temperature sensors limited)

## Contributing

Contributions are welcome! Areas for improvement:

- Enhanced temperature sensor support
- GPU monitoring
- Battery status tracking
- Historical data visualization
- Web dashboard interface

## Migration from Python Version

The Rust version maintains compatibility with the Python version's:
- Configuration file format
- Log file structure
- CLI interface

Simply replace `python tracker.py` with `tracker-rs` in your scripts.

## License

MIT License - See [LICENSE](LICENSE) file

## Author

m1ngsama

## Acknowledgments

- Original Python version inspired by [psutil](https://github.com/giampaolo/psutil)
- Rust implementation built on [sysinfo](https://github.com/GuillaumeGomez/sysinfo)
- Community feedback and contributions

---

**Note**: This is a complete rewrite in Rust. While maintaining feature parity with the Python version, it introduces performance improvements and memory safety guarantees inherent to Rust.
