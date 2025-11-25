#!/usr/bin/env python3
"""
System Tracker - Monitor machine health and performance
"""

import psutil
import time
import argparse
from datetime import datetime
from process_monitor import ProcessMonitor
from temperature_monitor import TemperatureMonitor
from config_manager import Config
from alert_system import AlertSystem
from logger import TrackerLogger


class SystemTracker:
    def __init__(self, config_file='config.json'):
        self.start_time = time.time()
        self.config = Config(config_file)
        self.process_monitor = ProcessMonitor()
        self.temperature_monitor = TemperatureMonitor()
        self.alert_system = AlertSystem(self.config)
        self.logger = TrackerLogger()

    def get_cpu_usage(self):
        """Get current CPU usage percentage"""
        try:
            return psutil.cpu_percent(interval=1, percpu=False)
        except Exception as e:
            self.logger.log_error(f"Failed to get CPU usage: {e}")
            return 0.0

    def get_memory_info(self):
        """Get memory usage statistics"""
        try:
            mem = psutil.virtual_memory()
            return {
                'total': mem.total,
                'available': mem.available,
                'percent': mem.percent,
                'used': mem.used
            }
        except Exception as e:
            self.logger.log_error(f"Failed to get memory info: {e}")
            return {'total': 0, 'available': 0, 'percent': 0, 'used': 0}

    def get_disk_usage(self):
        """Get disk usage statistics"""
        try:
            disk = psutil.disk_usage('/')
            return {
                'total': disk.total,
                'used': disk.used,
                'free': disk.free,
                'percent': disk.percent
            }
        except Exception as e:
            self.logger.log_error(f"Failed to get disk usage: {e}")
            return {'total': 0, 'used': 0, 'free': 0, 'percent': 0}

    def get_network_stats(self):
        """Get network I/O statistics"""
        try:
            net = psutil.net_io_counters()
            return {
                'bytes_sent': net.bytes_sent,
                'bytes_recv': net.bytes_recv,
                'packets_sent': net.packets_sent,
                'packets_recv': net.packets_recv
            }
        except Exception as e:
            self.logger.log_error(f"Failed to get network stats: {e}")
            return {'bytes_sent': 0, 'bytes_recv': 0, 'packets_sent': 0, 'packets_recv': 0}

    def display_stats(self):
        """Display all system statistics"""
        print(f"\n{'='*50}")
        print(f"System Tracker - {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
        print(f"{'='*50}\n")

        # CPU monitoring
        if self.config.get('display.show_cpu', True):
            cpu_usage = self.get_cpu_usage()
            print(f"CPU Usage: {cpu_usage}%")
            self.logger.log_stats('CPU', f"{cpu_usage}%")
            self.alert_system.check_cpu_alert(cpu_usage)

        # Memory monitoring
        if self.config.get('display.show_memory', True):
            mem = self.get_memory_info()
            print(f"Memory: {mem['percent']}% ({mem['used'] / (1024**3):.2f}GB / {mem['total'] / (1024**3):.2f}GB)")
            self.logger.log_stats('Memory', f"{mem['percent']}%")
            self.alert_system.check_memory_alert(mem['percent'])

        # Disk monitoring
        if self.config.get('display.show_disk', True):
            disk = self.get_disk_usage()
            print(f"Disk: {disk['percent']}% ({disk['used'] / (1024**3):.2f}GB / {disk['total'] / (1024**3):.2f}GB)")
            self.logger.log_stats('Disk', f"{disk['percent']}%")
            self.alert_system.check_disk_alert(disk['percent'])

        # Network monitoring
        if self.config.get('display.show_network', True):
            net = self.get_network_stats()
            print(f"Network: Sent {net['bytes_sent'] / (1024**2):.2f}MB | Recv {net['bytes_recv'] / (1024**2):.2f}MB")
            self.logger.log_stats('Network', f"Sent: {net['bytes_sent']} Recv: {net['bytes_recv']}")

        # Process monitoring
        if self.config.get('display.show_processes', True):
            self.process_monitor.display_processes()

        # Temperature monitoring
        if self.config.get('display.show_temperatures', True):
            self.temperature_monitor.display_temperatures()


def main():
    """Main entry point for the tracker application"""
    parser = argparse.ArgumentParser(description='System Tracker - Monitor machine health')
    parser.add_argument('-c', '--continuous', action='store_true', help='Run continuously')
    parser.add_argument('-i', '--interval', type=int, default=5, help='Update interval in seconds')
    args = parser.parse_args()

    tracker = SystemTracker()

    if args.continuous:
        try:
            while True:
                tracker.display_stats()
                time.sleep(args.interval)
        except KeyboardInterrupt:
            print("\n\nTracker stopped by user")
    else:
        tracker.display_stats()


if __name__ == "__main__":
    main()
