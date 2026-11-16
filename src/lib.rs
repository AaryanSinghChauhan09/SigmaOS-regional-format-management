// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-regional-format-management: regional format management component model.
//!
//! Real implementation: formats, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Format {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegionalFormatManagementError {
    EmptyFormatName,
    DuplicateFormat(String),
    EmptyEventSubject,
}

impl fmt::Display for RegionalFormatManagementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyFormatName => write!(f, "format name must not be empty"),
            Self::DuplicateFormat(id) => write!(f, "duplicate format {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for RegionalFormatManagementError {}

pub struct RegionalFormatManagement {
    pub name: String,
    formats: BTreeMap<String, Format>,
    events: BTreeMap<String, Event>,
}

impl RegionalFormatManagement {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            formats: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_format(&mut self, item: Format) -> Result<(), RegionalFormatManagementError> {
        if item.name.trim().is_empty() {
            return Err(RegionalFormatManagementError::EmptyFormatName);
        }
        let id = item.id.clone();
        if self.formats.contains_key(&id) {
            return Err(RegionalFormatManagementError::DuplicateFormat(id.clone()));
        }
        self.formats.insert(id, item);
        Ok(())
    }

    pub fn format_count(&self) -> usize {
        self.formats.len()
    }

    pub fn get_format(&self, id: &str) -> Option<&Format> {
        self.formats.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), RegionalFormatManagementError> {
        if event.subject.trim().is_empty() {
            return Err(RegionalFormatManagementError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_format(id: &str, name: &str) -> Format {
        Format {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = RegionalFormatManagement::new("RegionalFormatManagement");
        assert_eq!(m.name, "RegionalFormatManagement");
        assert_eq!(m.format_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_format() {
        let mut m = RegionalFormatManagement::new("RegionalFormatManagement");
        m.add_format(sample_format("a1", "Alpha")).unwrap();
        assert_eq!(m.format_count(), 1);
        assert!(m.get_format("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = RegionalFormatManagement::new("RegionalFormatManagement");
        assert!(m.add_format(sample_format("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = RegionalFormatManagement::new("RegionalFormatManagement");
        m.add_format(sample_format("x", "One")).unwrap();
        assert!(m.add_format(sample_format("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = RegionalFormatManagement::new("RegionalFormatManagement");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
