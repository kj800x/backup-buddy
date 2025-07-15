use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ParsePolicyKindError;

impl std::fmt::Display for ParsePolicyKindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid policy kind")
    }
}

impl std::error::Error for ParsePolicyKindError {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PolicyKind {
    Backup,
    Exclude,
    Null,
}

impl FromStr for PolicyKind {
    type Err = ParsePolicyKindError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "backup" => Ok(PolicyKind::Backup),
            "exclude" => Ok(PolicyKind::Exclude),
            "null" => Ok(PolicyKind::Null),
            _ => Err(ParsePolicyKindError),
        }
    }
}

impl ToString for PolicyKind {
    fn to_string(&self) -> String {
        match self {
            PolicyKind::Backup => "backup".to_string(),
            PolicyKind::Exclude => "exclude".to_string(),
            PolicyKind::Null => "null".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct BackupPolicy {
    pub id: Uuid,
    pub path: String,
    /// max age between backups in milliseconds
    pub max_staleness: u64,
    pub kind: PolicyKind,
    pub recursive: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum BackupAttemptStatus {
    Succeeded,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackupAttempt {
    pub id: Uuid,
    pub policy_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    pub status: BackupAttemptStatus,
}
