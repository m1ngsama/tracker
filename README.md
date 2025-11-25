# Tracker

A comprehensive system monitoring tool for tracking various machine health metrics and performance indicators.

[![CI](https://github.com/m1ngsama/tracker/actions/workflows/ci.yml/badge.svg)](https://github.com/m1ngsama/tracker/actions/workflows/ci.yml)
[![Release](https://github.com/m1ngsama/tracker/actions/workflows/release.yml/badge.svg)](https://github.com/m1ngsama/tracker/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **CPU Monitoring**: Real-time CPU usage percentage tracking
- **Memory Utilization**: Track memory usage with detailed statistics
- **Disk I/O Statistics**: Monitor disk usage and I/O operations
- **Network Traffic Analysis**: Track network bytes sent/received
- **Process Monitoring**: View top processes by CPU usage
- **Temperature Sensors**: Monitor system temperatures (if available)
- **Alert System**: Configurable thresholds for CPU, memory, and disk alerts
- **Logging**: Automatic logging of all metrics to daily log files
- **Data Export**: Export monitoring data to JSON or CSV formats
- **Configuration**: Customizable settings via JSON config file

## Installation

### From PyPI (coming soon)

```bash
pip install system-tracker
```

### From Source

```bash
git clone https://github.com/m1ngsama/tracker.git
cd tracker
pip install -r requirements.txt
```

## Usage

### Basic usage:
```bash
python tracker.py
```

### Continuous monitoring mode:
```bash
python tracker.py --continuous --interval 5
```

### Command line options:
- `-c, --continuous`: Run in continuous monitoring mode
- `-i, --interval`: Set update interval in seconds (default: 5)

## Configuration

The `config.json` file allows you to customize the tracker behavior:

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
    "cpu_percent": 80,
    "memory_percent": 85,
    "disk_percent": 90
  }
}
```

## Output

The tracker provides:
- **Console Output**: Real-time metrics displayed in the terminal
- **Log Files**: Daily logs stored in `logs/` directory
- **Alerts**: Visual and logged warnings when thresholds are exceeded
- **Export Data**: Optional data export to `exports/` directory

## Requirements

- Python 3.8+
- psutil
- GPUtil (for GPU monitoring)
- requests

## Development

### Running Tests

```bash
python test_tracker.py
```

### Project Structure

```
tracker/
├── tracker.py              # Main application
├── process_monitor.py      # Process monitoring module
├── temperature_monitor.py  # Temperature sensors module
├── config_manager.py       # Configuration management
├── alert_system.py         # Alert and threshold management
├── logger.py              # Logging functionality
├── data_exporter.py       # Data export utilities
├── config.json            # Configuration file
└── requirements.txt       # Python dependencies
```

## CI/CD

This project uses GitHub Actions for:
- **Continuous Integration**: Automated testing on multiple OS and Python versions
- **Automated Releases**: Automatic package building and release creation on version tags
- **Code Quality**: Linting and syntax checking

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see [LICENSE](LICENSE) file for details

## Author

m1ngsama

## Acknowledgments

- Built with [psutil](https://github.com/giampaolo/psutil) for cross-platform system monitoring
