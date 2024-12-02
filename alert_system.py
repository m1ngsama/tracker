"""
Alert system for system monitoring
"""

from logger import TrackerLogger


class AlertSystem:
    def __init__(self, config):
        self.config = config
        self.logger = TrackerLogger()
        self.alert_history = []

    def check_cpu_alert(self, cpu_percent):
        """Check if CPU usage exceeds threshold"""
        threshold = self.config.get('alert_thresholds.cpu_percent', 80)
        if cpu_percent > threshold:
            message = f"CPU usage is {cpu_percent}% (threshold: {threshold}%)"
            self.trigger_alert('CPU', message)
            return True
        return False

    def check_memory_alert(self, memory_percent):
        """Check if memory usage exceeds threshold"""
        threshold = self.config.get('alert_thresholds.memory_percent', 85)
        if memory_percent > threshold:
            message = f"Memory usage is {memory_percent}% (threshold: {threshold}%)"
            self.trigger_alert('Memory', message)
            return True
        return False

    def check_disk_alert(self, disk_percent):
        """Check if disk usage exceeds threshold"""
        threshold = self.config.get('alert_thresholds.disk_percent', 90)
        if disk_percent > threshold:
            message = f"Disk usage is {disk_percent}% (threshold: {threshold}%)"
            self.trigger_alert('Disk', message)
            return True
        return False

    def trigger_alert(self, alert_type, message):
        """Trigger an alert"""
        alert = {
            'type': alert_type,
            'message': message
        }
        self.alert_history.append(alert)
        self.logger.log_alert(f"{alert_type}: {message}")
        print(f"\n⚠️  ALERT: {message}")

    def get_alert_history(self):
        """Get alert history"""
        return self.alert_history
