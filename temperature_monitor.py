"""
Temperature sensor monitoring
"""

import psutil


class TemperatureMonitor:
    def get_temperatures(self):
        """Get system temperatures if available"""
        try:
            temps = psutil.sensors_temperatures()
            return temps
        except AttributeError:
            return None

    def display_temperatures(self):
        """Display system temperatures"""
        temps = self.get_temperatures()

        if not temps:
            print("\nTemperature sensors not available on this system")
            return

        print("\nSystem Temperatures:")
        print(f"{'Sensor':<30}{'Current':<15}{'High':<15}{'Critical':<15}")
        print("-" * 75)

        for name, entries in temps.items():
            for entry in entries:
                label = entry.label or name
                current = f"{entry.current}°C" if entry.current else "N/A"
                high = f"{entry.high}°C" if entry.high else "N/A"
                critical = f"{entry.critical}°C" if entry.critical else "N/A"
                print(f"{label:<30}{current:<15}{high:<15}{critical:<15}")
