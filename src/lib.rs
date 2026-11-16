// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-service-files: service files component model.
//!
//! Real implementation: files, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
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
pub enum ServiceFilesError {
    EmptyFileName,
    DuplicateFile(String),
    EmptyEventSubject,
}

impl fmt::Display for ServiceFilesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyFileName => write!(f, "file name must not be empty"),
            Self::DuplicateFile(id) => write!(f, "duplicate file {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ServiceFilesError {}

pub struct ServiceFiles {
    pub name: String,
    files: BTreeMap<String, File>,
    events: BTreeMap<String, Event>,
}

impl ServiceFiles {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            files: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_file(&mut self, item: File) -> Result<(), ServiceFilesError> {
        if item.name.trim().is_empty() {
            return Err(ServiceFilesError::EmptyFileName);
        }
        let id = item.id.clone();
        if self.files.contains_key(&id) {
            return Err(ServiceFilesError::DuplicateFile(id.clone()));
        }
        self.files.insert(id, item);
        Ok(())
    }

    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    pub fn get_file(&self, id: &str) -> Option<&File> {
        self.files.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ServiceFilesError> {
        if event.subject.trim().is_empty() {
            return Err(ServiceFilesError::EmptyEventSubject);
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

    fn sample_file(id: &str, name: &str) -> File {
        File {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ServiceFiles::new("ServiceFiles");
        assert_eq!(m.name, "ServiceFiles");
        assert_eq!(m.file_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_file() {
        let mut m = ServiceFiles::new("ServiceFiles");
        m.add_file(sample_file("a1", "Alpha")).unwrap();
        assert_eq!(m.file_count(), 1);
        assert!(m.get_file("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ServiceFiles::new("ServiceFiles");
        assert!(m.add_file(sample_file("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ServiceFiles::new("ServiceFiles");
        m.add_file(sample_file("x", "One")).unwrap();
        assert!(m.add_file(sample_file("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ServiceFiles::new("ServiceFiles");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
