#!/usr/bin/env python3
"""
System Tracker - Monitor machine health and performance
"""

import psutil
import time
from datetime import datetime


class SystemTracker:
    def __init__(self):
        self.start_time = time.time()

    def get_cpu_usage(self):
        """Get current CPU usage percentage"""
        return psutil.cpu_percent(interval=1)

    def get_memory_info(self):
        """Get memory usage statistics"""
        mem = psutil.virtual_memory()
        return {
            'total': mem.total,
            'available': mem.available,
            'percent': mem.percent,
            'used': mem.used
        }

    def get_disk_usage(self):
        """Get disk usage statistics"""
        disk = psutil.disk_usage('/')
        return {
            'total': disk.total,
            'used': disk.used,
            'free': disk.free,
            'percent': disk.percent
        }

    def display_stats(self):
        """Display all system statistics"""
        print(f"\n{'='*50}")
        print(f"System Tracker - {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
        print(f"{'='*50}\n")

        print(f"CPU Usage: {self.get_cpu_usage()}%")

        mem = self.get_memory_info()
        print(f"Memory: {mem['percent']}% ({mem['used'] / (1024**3):.2f}GB / {mem['total'] / (1024**3):.2f}GB)")

        disk = self.get_disk_usage()
        print(f"Disk: {disk['percent']}% ({disk['used'] / (1024**3):.2f}GB / {disk['total'] / (1024**3):.2f}GB)")


if __name__ == "__main__":
    tracker = SystemTracker()
    tracker.display_stats()
