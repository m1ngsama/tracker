use crossterm::style::Stylize;

pub struct TemperatureMonitor;

impl TemperatureMonitor {
    pub fn new() -> Self {
        TemperatureMonitor
    }

    pub fn display_temperatures(&mut self) {
        println!("{}", "Temperature sensors not available on this system".yellow());
    }
}

impl Default for TemperatureMonitor {
    fn default() -> Self {
        Self::new()
    }
}
