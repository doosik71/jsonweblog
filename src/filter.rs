use crate::{LogEntry, LogLevel};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogFilter {
    pub level: Option<LogLevel>,
    pub search_text: Option<String>,
    pub logger: Option<String>,
    pub module: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

impl LogFilter {
    pub fn new() -> Self {
        Self {
            level: None,
            search_text: None,
            logger: None,
            module: None,
            start_time: None,
            end_time: None,
        }
    }

    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.level = Some(level);
        self
    }

    pub fn with_search_text(mut self, text: String) -> Self {
        self.search_text = Some(text);
        self
    }

    pub fn with_logger(mut self, logger: String) -> Self {
        self.logger = Some(logger);
        self
    }

    pub fn with_module(mut self, module: String) -> Self {
        self.module = Some(module);
        self
    }

    pub fn with_time_range(mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        self.start_time = Some(start);
        self.end_time = Some(end);
        self
    }

    pub fn matches(&self, entry: &LogEntry) -> bool {
        // Level filter
        if let Some(filter_level) = &self.level {
            if entry.level != *filter_level {
                return false;
            }
        }

        // Search text filter (case-insensitive)
        if let Some(search_text) = &self.search_text {
            let search_lower = search_text.to_lowercase();
            let message_matches = entry.message.to_lowercase().contains(&search_lower);
            let logger_matches = entry.logger.to_lowercase().contains(&search_lower);
            let module_matches = entry.module
                .as_ref()
                .map(|m| m.to_lowercase().contains(&search_lower))
                .unwrap_or(false);
            let function_matches = entry.function
                .as_ref()
                .map(|f| f.to_lowercase().contains(&search_lower))
                .unwrap_or(false);

            if !message_matches && !logger_matches && !module_matches && !function_matches {
                return false;
            }
        }

        // Logger filter
        if let Some(filter_logger) = &self.logger {
            if !entry.logger.contains(filter_logger) {
                return false;
            }
        }

        // Module filter
        if let Some(filter_module) = &self.module {
            if let Some(entry_module) = &entry.module {
                if !entry_module.contains(filter_module) {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Time range filter
        if let Some(start_time) = &self.start_time {
            if entry.timestamp < *start_time {
                return false;
            }
        }

        if let Some(end_time) = &self.end_time {
            if entry.timestamp > *end_time {
                return false;
            }
        }

        true
    }

    pub fn is_empty(&self) -> bool {
        self.level.is_none()
            && self.search_text.is_none()
            && self.logger.is_none()
            && self.module.is_none()
            && self.start_time.is_none()
            && self.end_time.is_none()
    }

    pub fn clear(&mut self) {
        self.level = None;
        self.search_text = None;
        self.logger = None;
        self.module = None;
        self.start_time = None;
        self.end_time = None;
    }
}

impl Default for LogFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_empty_filter_matches_all() {
        let filter = LogFilter::new();
        let entry = LogEntry::new(
            1,
            Utc::now(),
            LogLevel::Info,
            "test".to_string(),
            "test message".to_string(),
        );
        
        assert!(filter.matches(&entry));
    }

    #[test]
    fn test_level_filter() {
        let filter = LogFilter::new().with_level(LogLevel::Error);
        
        let info_entry = LogEntry::new(
            1,
            Utc::now(),
            LogLevel::Info,
            "test".to_string(),
            "test message".to_string(),
        );
        
        let error_entry = LogEntry::new(
            2,
            Utc::now(),
            LogLevel::Error,
            "test".to_string(),
            "error message".to_string(),
        );
        
        assert!(!filter.matches(&info_entry));
        assert!(filter.matches(&error_entry));
    }

    #[test]
    fn test_search_text_filter() {
        let filter = LogFilter::new().with_search_text("error".to_string());
        
        let matching_entry = LogEntry::new(
            1,
            Utc::now(),
            LogLevel::Info,
            "test".to_string(),
            "This is an error message".to_string(),
        );
        
        let non_matching_entry = LogEntry::new(
            2,
            Utc::now(),
            LogLevel::Info,
            "test".to_string(),
            "This is a normal message".to_string(),
        );
        
        assert!(filter.matches(&matching_entry));
        assert!(!filter.matches(&non_matching_entry));
    }
}