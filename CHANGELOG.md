# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-11-25

### Added
- Initial release of System Tracker
- Real-time CPU usage monitoring
- Memory utilization tracking with detailed statistics
- Disk I/O statistics and usage monitoring
- Network traffic analysis (bytes sent/received, packets)
- Process monitoring with top processes by CPU usage
- Temperature sensor monitoring (platform-dependent)
- Configurable alert system with thresholds for CPU, memory, and disk
- Comprehensive logging system with daily log files
- Data export functionality (JSON and CSV formats)
- Configuration management via `config.json`
- Continuous monitoring mode with customizable intervals
- CLI arguments support for flexible operation
- Cross-platform support (Linux, macOS, Windows)

### Features
- **Configuration System**: JSON-based configuration with sensible defaults
- **Alert System**: Real-time alerts when system metrics exceed configured thresholds
- **Logging**: Automatic daily log file creation in `logs/` directory
- **Data Export**: Export monitoring data to `exports/` directory
- **Process Monitor**: Enhanced CPU usage tracking with accurate process information
- **Temperature Monitoring**: System temperature sensors (when available)
- **Error Handling**: Comprehensive error handling throughout the codebase
- **Modular Architecture**: Clean separation of concerns with dedicated modules

### Technical Improvements
- Fixed data exporter directory creation issue
- Improved process monitor CPU data accuracy with proper interval handling
- Added error handling to all system metric collection methods
- Resolved logger file handle management issues
- Enhanced zombie process handling in process monitoring

### CI/CD
- GitHub Actions workflow for automated testing across multiple OS and Python versions
- Automated release workflow with package building
- Code quality checks with flake8 linting
- Multi-platform testing (Ubuntu, macOS, Windows)
- Python 3.8-3.12 compatibility testing

### Documentation
- Comprehensive README with installation and usage instructions
- Configuration file documentation
- MIT License
- Project structure documentation
- Contributing guidelines

### Dependencies
- psutil >= 5.9.0
- GPUtil >= 1.4.0
- requests >= 2.28.0

[1.0.0]: https://github.com/m1ngsama/tracker/releases/tag/v1.0.0
