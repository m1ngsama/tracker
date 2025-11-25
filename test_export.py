#!/usr/bin/env python3
"""Test data export functionality"""

from data_exporter import DataExporter

# Test data
test_data = [
    {'timestamp': '2025-11-25 15:00:00', 'cpu': 45.2, 'memory': 60.1},
    {'timestamp': '2025-11-25 15:05:00', 'cpu': 52.3, 'memory': 62.5},
    {'timestamp': '2025-11-25 15:10:00', 'cpu': 48.9, 'memory': 61.8}
]

exporter = DataExporter()

# Test JSON export
json_file = exporter.export_to_json(test_data)
print(f"✓ JSON export successful: {json_file}")

# Test CSV export
csv_file = exporter.export_to_csv(test_data)
print(f"✓ CSV export successful: {csv_file}")

print("\nExport directory contents:")
import os
for file in os.listdir('exports'):
    print(f"  - {file}")
