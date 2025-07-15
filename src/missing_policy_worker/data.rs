use std::{collections::HashMap, path::PathBuf};

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct MissingPolicyReport {
    pub missing_paths: Vec<PathBuf>,
    pub policy_hits: HashMap<Uuid, u64>,
}

#[derive(Debug, Clone)]
pub struct MissingPolicyWorkerData {
    pub report: Option<MissingPolicyReport>,
    pub last_run_started: Option<DateTime<Utc>>,
    pub last_run_completed: Option<DateTime<Utc>>,
}

impl MissingPolicyWorkerData {
    pub fn new() -> Self {
        Self {
            report: None,
            last_run_started: None,
            last_run_completed: None,
        }
    }
}
