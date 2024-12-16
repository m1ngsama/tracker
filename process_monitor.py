"""
Process monitoring utilities
"""

import psutil


class ProcessMonitor:
    def get_top_processes(self, limit=5):
        """Get top processes by CPU usage"""
        processes = []
        for proc in psutil.process_iter(['pid', 'name', 'cpu_percent', 'memory_percent']):
            try:
                processes.append(proc.info)
            except (psutil.NoSuchProcess, psutil.AccessDenied):
                pass

        processes.sort(key=lambda x: x['cpu_percent'] or 0, reverse=True)
        return processes[:limit]

    def get_process_count(self):
        """Get total number of running processes"""
        return len(psutil.pids())

    def display_processes(self):
        """Display top processes"""
        print(f"\nTop Processes by CPU Usage:")
        print(f"{'PID':<10}{'Name':<30}{'CPU%':<10}{'Memory%':<10}")
        print("-" * 60)

        for proc in self.get_top_processes():
            cpu = proc['cpu_percent'] if proc['cpu_percent'] is not None else 0
            mem = proc['memory_percent'] if proc['memory_percent'] is not None else 0
            print(f"{proc['pid']:<10}{proc['name']:<30}{cpu:<10.2f}{mem:<10.2f}")

        print(f"\nTotal Processes: {self.get_process_count()}")
