use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub line: u64,
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub logger: String,
    pub message: String,
    pub module: Option<String>,
    pub function: Option<String>,
    pub raw_fields: HashMap<String, serde_json::Value>,
    pub dynamic_fields: HashMap<String, DynamicFieldValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DynamicFieldValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Object(serde_json::Value),
    Array(Vec<serde_json::Value>),
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

impl LogLevel {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "TRACE" => LogLevel::Trace,
            "DEBUG" => LogLevel::Debug,
            "INFO" => LogLevel::Info,
            "WARN" | "WARNING" => LogLevel::Warn,
            "ERROR" => LogLevel::Error,
            "FATAL" | "CRITICAL" => LogLevel::Fatal,
            _ => LogLevel::Info,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            LogLevel::Trace => "#6B7280",
            LogLevel::Debug => "#3B82F6",
            LogLevel::Info => "#10B981",
            LogLevel::Warn => "#F59E0B",
            LogLevel::Error => "#EF4444",
            LogLevel::Fatal => "#DC2626",
        }
    }
}

impl LogEntry {
    pub fn new(
        line: u64,
        timestamp: DateTime<Utc>,
        level: LogLevel,
        logger: String,
        message: String,
    ) -> Self {
        Self {
            line,
            timestamp,
            level,
            logger,
            message,
            module: None,
            function: None,
            raw_fields: HashMap::new(),
            dynamic_fields: HashMap::new(),
        }
    }

    pub fn with_module(mut self, module: String) -> Self {
        self.module = Some(module);
        self
    }

    pub fn with_function(mut self, function: String) -> Self {
        self.function = Some(function);
        self
    }

    pub fn with_raw_fields(mut self, fields: HashMap<String, serde_json::Value>) -> Self {
        self.raw_fields = fields.clone();
        self.dynamic_fields = Self::extract_dynamic_fields(&fields);
        self
    }

    pub fn extract_dynamic_fields(fields: &HashMap<String, serde_json::Value>) -> HashMap<String, DynamicFieldValue> {
        let mut dynamic_fields = HashMap::new();
        Self::flatten_object(fields, "", &mut dynamic_fields);
        dynamic_fields
    }

    fn flatten_object(
        obj: &HashMap<String, serde_json::Value>,
        prefix: &str,
        result: &mut HashMap<String, DynamicFieldValue>,
    ) {
        for (key, value) in obj {
            let field_path = if prefix.is_empty() {
                key.clone()
            } else {
                format!("{}.{}", prefix, key)
            };

            Self::flatten_value(&field_path, value, result);
        }
    }

    fn flatten_value(
        path: &str,
        value: &serde_json::Value,
        result: &mut HashMap<String, DynamicFieldValue>,
    ) {
        match value {
            serde_json::Value::Null => {
                result.insert(path.to_string(), DynamicFieldValue::Null);
            }
            serde_json::Value::Bool(b) => {
                result.insert(path.to_string(), DynamicFieldValue::Boolean(*b));
            }
            serde_json::Value::Number(n) => {
                if let Some(f) = n.as_f64() {
                    result.insert(path.to_string(), DynamicFieldValue::Number(f));
                }
            }
            serde_json::Value::String(s) => {
                result.insert(path.to_string(), DynamicFieldValue::String(s.clone()));
            }
            serde_json::Value::Array(arr) => {
                result.insert(path.to_string(), DynamicFieldValue::Array(arr.clone()));
                
                // Also flatten array elements
                for (index, item) in arr.iter().enumerate().take(10) { // Limit to first 10 elements
                    let array_path = format!("{}[{}]", path, index);
                    Self::flatten_value(&array_path, item, result);
                }
            }
            serde_json::Value::Object(_obj) => {
                result.insert(path.to_string(), DynamicFieldValue::Object(value.clone()));
                
                // Recursively flatten nested objects
                if let Ok(obj_map) = serde_json::from_value::<HashMap<String, serde_json::Value>>(value.clone()) {
                    Self::flatten_object(&obj_map, path, result);
                }
            }
        }
    }

    pub fn get_field_value(&self, field_path: &str) -> Option<&DynamicFieldValue> {
        self.dynamic_fields.get(field_path)
    }

    pub fn get_field_as_string(&self, field_path: &str) -> Option<String> {
        self.get_field_value(field_path).map(|value| match value {
            DynamicFieldValue::String(s) => s.clone(),
            DynamicFieldValue::Number(n) => n.to_string(),
            DynamicFieldValue::Boolean(b) => b.to_string(),
            DynamicFieldValue::Null => "null".to_string(),
            DynamicFieldValue::Object(obj) => serde_json::to_string(obj).unwrap_or_default(),
            DynamicFieldValue::Array(arr) => serde_json::to_string(arr).unwrap_or_default(),
        })
    }
}