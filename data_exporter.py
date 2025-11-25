"""
Data export functionality
"""

import json
import csv
from datetime import datetime


class DataExporter:
    def __init__(self, output_dir='exports'):
        self.output_dir = output_dir
        self._ensure_directory()

    def _ensure_directory(self):
        """Ensure the output directory exists"""
        import os
        if not os.path.exists(self.output_dir):
            os.makedirs(self.output_dir)

    def export_to_json(self, data, filename=None):
        """Export data to JSON format"""
        if filename is None:
            filename = f"tracker_data_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"

        filepath = f"{self.output_dir}/{filename}"

        try:
            with open(filepath, 'w') as f:
                json.dump(data, f, indent=2)
            return filepath
        except IOError as e:
            raise IOError(f"Failed to export data to JSON: {e}")

    def export_to_csv(self, data, filename=None):
        """Export data to CSV format"""
        if filename is None:
            filename = f"tracker_data_{datetime.now().strftime('%Y%m%d_%H%M%S')}.csv"

        filepath = f"{self.output_dir}/{filename}"

        try:
            if isinstance(data, list) and len(data) > 0:
                keys = data[0].keys()
                with open(filepath, 'w', newline='') as f:
                    writer = csv.DictWriter(f, fieldnames=keys)
                    writer.writeheader()
                    writer.writerows(data)
            return filepath
        except IOError as e:
            raise IOError(f"Failed to export data to CSV: {e}")
