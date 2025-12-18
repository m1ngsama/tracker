tracker-rs(1)                         System Tracker                         tracker-rs(1)

NAME
       tracker-rs - high-performance system monitoring tool

SYNOPSIS
       tracker-rs [OPTIONS]

DESCRIPTION
       tracker-rs is a high-performance, memory-safe system monitoring tool written in Rust.
       It provides real-time statistics for CPU, memory, disk I/O, and network traffic, along
       with process management and temperature monitoring.

       Designed as a drop-in replacement for the legacy Python implementation, it offers
       significant performance improvements (10-50x faster execution, reduced memory usage)
       while maintaining configuration compatibility.

OPTIONS
       -c, --continuous
              Run in continuous monitoring mode. The tool will repeatedly display statistics
              and log data based on the update interval.

       -i, --interval SECONDS
              Set the update interval in seconds. Default is 5 seconds.
              This option is primarily used with --continuous mode.

       --config FILE
              Path to the configuration file. Default is "config.json".
              If the file does not exist, internal defaults are used.

       -h, --help
              Print help message and exit.

       -V, --version
              Print version information and exit.

CONFIGURATION
       The tool is configured via a JSON file (default: config.json).
       The configuration file supports the following keys:

       update_interval (integer)
              Default refresh rate in seconds (overridden by -i).

       display (object)
              Toggle individual monitoring features on/off:
              - show_cpu (boolean)
              - show_memory (boolean)
              - show_disk (boolean)
              - show_network (boolean)
              - show_processes (boolean)
              - show_temperatures (boolean)

       process_limit (integer)
              Number of top CPU-consuming processes to display.

       alert_thresholds (object)
              Percentage thresholds for triggering alerts:
              - cpu_percent (float)
              - memory_percent (float)
              - disk_percent (float)

       Example config.json:
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

OUTPUT
       tracker-rs outputs formatted system statistics to the console.
       In continuous mode, it updates at the specified interval.

       CPU Usage: 35.42%
       Memory: 58.21% (14.00GB / 24.00GB)
       Disk: 50.40% (464.07GB / 920.86GB)
       Network: Sent 4872.76MB | Recv 6633.56MB

       Top Processes by CPU Usage:
       PID       Name                          CPU%      Memory%
       ------------------------------------------------------------
       1234      chrome                        45.23     3.21

FILES
       config.json
              Default configuration file location.

       logs/tracker_YYYYMMDD.log
              Daily rotating log files containing system stats and alerts.

EXIT STATUS
       0      Success.
       1      Failure (e.g., invalid configuration).

EXAMPLES
       Run a single monitoring snapshot:
              $ tracker-rs

       Run continuously every 2 seconds:
              $ tracker-rs -c -i 2

       Use a custom configuration file:
              $ tracker-rs --config /etc/tracker/config.json

SEE ALSO
       top(1), htop(1), ps(1)

BUGS
       Report bugs to https://github.com/m1ngsama/tracker/issues

AUTHOR
       m1ngsama

LICENSE
       MIT License

v1.0.1                                2025-12-18                         tracker-rs(1)