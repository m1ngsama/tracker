"""
Logging functionality for tracker
"""

import logging
import os
from datetime import datetime


class TrackerLogger:
    def __init__(self, log_dir='logs'):
        self.log_dir = log_dir
        self.setup_logger()

    def setup_logger(self):
        """Setup logging configuration"""
        if not os.path.exists(self.log_dir):
            os.makedirs(self.log_dir)

        log_file = os.path.join(
            self.log_dir,
            f"tracker_{datetime.now().strftime('%Y%m%d')}.log"
        )

        logging.basicConfig(
            level=logging.INFO,
            format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
            handlers=[
                logging.FileHandler(log_file),
                logging.StreamHandler()
            ]
        )

        self.logger = logging.getLogger('SystemTracker')

    def log_stats(self, stats_type, stats_data):
        """Log system statistics"""
        self.logger.info(f"{stats_type}: {stats_data}")

    def log_alert(self, message):
        """Log alert messages"""
        self.logger.warning(f"ALERT: {message}")

    def log_error(self, error_message):
        """Log error messages"""
        self.logger.error(f"ERROR: {error_message}")
