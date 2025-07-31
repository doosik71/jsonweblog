use crate::{LogEntry, LogLevel};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_stream::{wrappers::LinesStream, StreamExt};

pub struct JsonLogParser {
    line_counter: u64,
}

impl JsonLogParser {
    pub fn new() -> Self {
        Self { line_counter: 0 }
    }

    pub async fn parse_stdin(&mut self) -> impl futures::Stream<Item = Result<LogEntry>> + '_ {
        let stdin = tokio::io::stdin();
        let reader = BufReader::new(stdin);
        let lines = LinesStream::new(reader.lines());

        lines.map(move |line_result: Result<String, std::io::Error>| {
            self.line_counter += 1;
            match line_result {
                Ok(line) => self.parse_line(&line, self.line_counter),
                Err(e) => Err(anyhow!("Failed to read line: {}", e)),
            }
        })
    }

    pub fn parse_line(&self, line: &str, line_number: u64) -> Result<LogEntry> {
        let line = line.trim();
        if line.is_empty() {
            return Err(anyhow!("Empty line"));
        }

        let json_value: Value = serde_json::from_str(line)
            .map_err(|e| anyhow!("Failed to parse JSON: {}", e))?;

        if let Value::Object(obj) = json_value {
            self.extract_log_entry(obj, line_number)
        } else {
            Err(anyhow!("Expected JSON object"))
        }
    }

    fn extract_log_entry(
        &self,
        obj: serde_json::Map<String, Value>,
        line_number: u64,
    ) -> Result<LogEntry> {
        // Store all raw fields for dynamic schema detection
        let raw_fields: HashMap<String, Value> = obj.clone().into_iter().collect();

        // Try to extract common fields with fallbacks, but don't assume they exist
        let timestamp = self.extract_timestamp(&obj).unwrap_or_else(|_| Utc::now());
        let level = self.extract_level(&obj);
        let logger = self.extract_string_field(&obj, &["logger", "logger_name", "name", "category", "component"])
            .unwrap_or_else(|| "unknown".to_string());
        let message = self.extract_string_field(&obj, &["message", "msg", "text", "description", "content"])
            .unwrap_or_else(|| "".to_string());

        // Extract optional fields with expanded search
        let module = self.extract_string_field(&obj, &["module", "mod", "component", "file", "filename"]);
        let function = self.extract_string_field(&obj, &["function", "func", "method", "procedure"]);

        let mut entry = LogEntry::new(line_number, timestamp, level, logger, message);
        
        if let Some(module) = module {
            entry = entry.with_module(module);
        }
        
        if let Some(function) = function {
            entry = entry.with_function(function);
        }
        
        // Store all raw fields for dynamic field extraction
        entry = entry.with_raw_fields(raw_fields);

        Ok(entry)
    }

    fn extract_timestamp(&self, obj: &serde_json::Map<String, Value>) -> Result<DateTime<Utc>> {
        let timestamp_keys = ["timestamp", "time", "ts", "@timestamp", "datetime", "created_at"];
        
        for key in &timestamp_keys {
            if let Some(value) = obj.get(*key) {
                return self.parse_timestamp(value);
            }
        }

        // If no timestamp found, return error (caller will use current time)
        Err(anyhow!("No timestamp field found"))
    }

    fn parse_timestamp(&self, value: &Value) -> Result<DateTime<Utc>> {
        match value {
            Value::String(s) => {
                // Try multiple timestamp formats
                let formats = [
                    "%Y-%m-%dT%H:%M:%S%.fZ",
                    "%Y-%m-%dT%H:%M:%SZ",
                    "%Y-%m-%d %H:%M:%S%.f",
                    "%Y-%m-%d %H:%M:%S",
                    "%d/%m/%Y %H:%M:%S",
                    "%m/%d/%Y %H:%M:%S",
                ];

                for format in &formats {
                    if let Ok(dt) = DateTime::parse_from_str(s, format) {
                        return Ok(dt.with_timezone(&Utc));
                    }
                }

                // Try parsing as RFC3339
                if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
                    return Ok(dt.with_timezone(&Utc));
                }

                Err(anyhow!("Unable to parse timestamp: {}", s))
            }
            Value::Number(n) => {
                if let Some(timestamp) = n.as_i64() {
                    // Assume Unix timestamp (seconds or milliseconds)
                    let dt = if timestamp > 1_000_000_000_000 {
                        // Milliseconds
                        DateTime::from_timestamp_millis(timestamp)
                    } else {
                        // Seconds
                        DateTime::from_timestamp(timestamp, 0)
                    };

                    dt.ok_or_else(|| anyhow!("Invalid Unix timestamp: {}", timestamp))
                } else {
                    Err(anyhow!("Invalid timestamp number format"))
                }
            }
            _ => Err(anyhow!("Timestamp must be string or number")),
        }
    }

    fn extract_level(&self, obj: &serde_json::Map<String, Value>) -> LogLevel {
        let level_keys = ["level", "lvl", "severity", "priority", "log_level"];
        
        for key in &level_keys {
            if let Some(value) = obj.get(*key) {
                if let Some(level_str) = value.as_str() {
                    return LogLevel::from_str(level_str);
                }
            }
        }

        LogLevel::Info
    }

    fn extract_string_field(
        &self,
        obj: &serde_json::Map<String, Value>,
        keys: &[&str],
    ) -> Option<String> {
        for key in keys {
            if let Some(value) = obj.get(*key) {
                if let Some(s) = value.as_str() {
                    return Some(s.to_string());
                }
            }
        }
        None
    }
}

impl Default for JsonLogParser {
    fn default() -> Self {
        Self::new()
    }
}