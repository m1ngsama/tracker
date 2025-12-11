pub struct TemperatureMonitor;

impl TemperatureMonitor {
    pub fn new() -> Self {
        TemperatureMonitor
    }

    pub fn display_temperatures(&mut self) {
        println!("\nTemperature sensors not available on this system");
    }
}

impl Default for TemperatureMonitor {
    fn default() -> Self {
        Self::new()
    }
}
