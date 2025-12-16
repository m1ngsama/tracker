#![allow(dead_code)]
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use anyhow::Result;
use chrono::Local;

pub struct DataExporter {
    output_dir: PathBuf,
}

impl DataExporter {
    pub fn new(output_dir: &str) -> Self {
        let output_dir = PathBuf::from(output_dir);
        if !output_dir.exists() {
            fs::create_dir_all(&output_dir).ok();
        }
        DataExporter { output_dir }
    }

    pub fn export_to_json<T: Serialize>(&self, data: &T, filename: Option<String>) -> Result<PathBuf> {
        let filename = filename.unwrap_or_else(|| {
            format!("tracker_data_{}.json", Local::now().format("%Y%m%d_%H%M%S"))
        });
        
        let filepath = self.output_dir.join(filename);
        let json = serde_json::to_string_pretty(data)?;
        fs::write(&filepath, json)?;
        
        Ok(filepath)
    }

    pub fn export_to_csv<T: Serialize>(&self, data: &[T], filename: Option<String>) -> Result<PathBuf> {
        let filename = filename.unwrap_or_else(|| {
            format!("tracker_data_{}.csv", Local::now().format("%Y%m%d_%H%M%S"))
        });
        
        let filepath = self.output_dir.join(filename);
        let mut wtr = csv::Writer::from_path(&filepath)?;
        
        for record in data {
            wtr.serialize(record)?;
        }
        
        wtr.flush()?;
        Ok(filepath)
    }
}

impl Default for DataExporter {
    fn default() -> Self {
        Self::new("exports")
    }
}
