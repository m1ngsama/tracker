"""
Configuration management for tracker
"""

import json
import os


class Config:
    def __init__(self, config_file='config.json'):
        self.config_file = config_file
        self.config = self.load_config()

    def load_config(self):
        """Load configuration from JSON file"""
        if os.path.exists(self.config_file):
            with open(self.config_file, 'r') as f:
                return json.load(f)
        return self.get_default_config()

    def get_default_config(self):
        """Return default configuration"""
        return {
            'update_interval': 5,
            'display': {
                'show_cpu': True,
                'show_memory': True,
                'show_disk': True,
                'show_network': True,
                'show_processes': True,
                'show_temperatures': True
            },
            'process_limit': 5,
            'alert_thresholds': {
                'cpu_percent': 80,
                'memory_percent': 85,
                'disk_percent': 90
            }
        }

    def get(self, key, default=None):
        """Get configuration value"""
        keys = key.split('.')
        value = self.config
        for k in keys:
            if isinstance(value, dict):
                value = value.get(k, default)
            else:
                return default
        return value
