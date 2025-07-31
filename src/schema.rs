use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub fields: Vec<String>, // Simple list of field names from first log entry
    pub initialized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnConfig {
    pub field_name: String,
    pub width: u32,
    pub visible: bool,
    pub order: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableConfig {
    pub theme: Option<String>,
    pub columns: Vec<ColumnConfig>,
}

impl Schema {
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
            initialized: false,
        }
    }

    pub fn initialize_from_first_entry(&mut self, raw_fields: &std::collections::HashMap<String, serde_json::Value>) {
        if !self.initialized {
            // Always start with line number field
            let mut fields = vec!["#".to_string()];
            // Add fields from raw_fields
            fields.extend(raw_fields.keys().cloned());
            self.fields = fields;
            self.initialized = true;
        }
    }

    pub fn get_default_columns(&self) -> Vec<ColumnConfig> {
        self.fields.iter().enumerate().map(|(index, field_name)| {
            let width = if field_name == "#" { 80 } else { 150 };
            ColumnConfig {
                field_name: field_name.clone(),
                width,
                visible: true,
                order: index,
            }
        }).collect()
    }
}

impl TableConfig {
    pub fn save_to_file(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        if path.exists() {
            let json = std::fs::read_to_string(path)?;
            let config: TableConfig = serde_json::from_str(&json)?;
            Ok(config)
        } else {
            Err("Settings file does not exist".into())
        }
    }

    pub fn get_settings_path() -> std::path::PathBuf {
        std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .join("jsonweblog_settings.json")
    }
}

impl Default for Schema {
    fn default() -> Self {
        Self::new()
    }
}