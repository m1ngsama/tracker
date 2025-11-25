"""
Process monitoring utilities
"""

import psutil


class ProcessMonitor:
    def get_top_processes(self, limit=5):
        """Get top processes by CPU usage"""
        processes = []
        # First collect all processes
        for proc in psutil.process_iter(['pid', 'name']):
            try:
                # Get CPU percent with interval for more accurate reading
                cpu_percent = proc.cpu_percent(interval=0.1)
                memory_percent = proc.memory_percent()
                processes.append({
                    'pid': proc.info['pid'],
                    'name': proc.info['name'],
                    'cpu_percent': cpu_percent,
                    'memory_percent': memory_percent
                })
            except (psutil.NoSuchProcess, psutil.AccessDenied, psutil.ZombieProcess):
                pass

        processes.sort(key=lambda x: x['cpu_percent'] or 0, reverse=True)
        return processes[:limit]

    def get_process_count(self):
        """Get total number of running processes"""
        try:
            return len(psutil.pids())
        except Exception:
            return 0

    def display_processes(self):
        """Display top processes"""
        try:
            print(f"\nTop Processes by CPU Usage:")
            print(f"{'PID':<10}{'Name':<30}{'CPU%':<10}{'Memory%':<10}")
            print("-" * 60)

            for proc in self.get_top_processes():
                cpu = proc['cpu_percent'] if proc['cpu_percent'] is not None else 0
                mem = proc['memory_percent'] if proc['memory_percent'] is not None else 0
                print(f"{proc['pid']:<10}{proc['name']:<30}{cpu:<10.2f}{mem:<10.2f}")

            print(f"\nTotal Processes: {self.get_process_count()}")
        except Exception as e:
            print(f"Error displaying processes: {e}")
