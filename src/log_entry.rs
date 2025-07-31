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
        self.raw_fields = fields;
        self
    }
}