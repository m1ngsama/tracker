# Tracker

A comprehensive system monitoring tool for tracking various machine health metrics and performance indicators.

## Features

- **CPU Monitoring**: Real-time CPU usage percentage tracking
- **Memory Utilization**: Track memory usage with detailed statistics
- **Disk I/O Statistics**: Monitor disk usage and I/O operations
- **Network Traffic Analysis**: Track network bytes sent/received
- **Process Monitoring**: View top processes by CPU usage
- **Temperature Sensors**: Monitor system temperatures (if available)
- **System Uptime Tracking**: Track how long the system has been running

## Installation

```bash
pip install -r requirements.txt
```

## Usage

Basic usage:
```bash
python tracker.py
```

Continuous monitoring mode:
```bash
python tracker.py --continuous --interval 5
```

Command line options:
- `-c, --continuous`: Run in continuous monitoring mode
- `-i, --interval`: Set update interval in seconds (default: 5)

## Requirements

- Python 3.8+
- psutil
- GPUtil (for GPU monitoring)

## License

MIT License
