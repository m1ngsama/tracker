"""
Unit tests for tracker functionality
"""

import unittest
from unittest.mock import Mock, patch
from tracker import SystemTracker


class TestSystemTracker(unittest.TestCase):
    def setUp(self):
        self.tracker = SystemTracker()

    @patch('psutil.cpu_percent')
    def test_get_cpu_usage(self, mock_cpu):
        mock_cpu.return_value = 50.0
        result = self.tracker.get_cpu_usage()
        self.assertEqual(result, 50.0)

    @patch('psutil.virtual_memory')
    def test_get_memory_info(self, mock_mem):
        mock_mem.return_value = Mock(
            total=8589934592,
            available=4294967296,
            percent=50.0,
            used=4294967296
        )
        result = self.tracker.get_memory_info()
        self.assertEqual(result['percent'], 50.0)
        self.assertEqual(result['total'], 8589934592)

    @patch('psutil.disk_usage')
    def test_get_disk_usage(self, mock_disk):
        mock_disk.return_value = Mock(
            total=1000000000000,
            used=500000000000,
            free=500000000000,
            percent=50.0
        )
        result = self.tracker.get_disk_usage()
        self.assertEqual(result['percent'], 50.0)


if __name__ == '__main__':
    unittest.main()
