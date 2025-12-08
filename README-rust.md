# Tracker (Rust)

A high-performance system monitoring tool written in Rust.

## Features

- CPU, memory, disk, and network monitoring
- Process tracking with top CPU consumers
- Temperature sensor support (platform-dependent)
- Configurable alert thresholds
- Automatic logging to daily log files
- Data export to JSON/CSV formats
- Cross-platform compatibility

## Installation

```bash
cargo build --release
```

## Usage

```bash
# Single run
cargo run

# Continuous monitoring
cargo run -- --continuous --interval 5
```

## Configuration

Edit `config.json` to customize monitoring behavior and alert thresholds.

## License

MIT
