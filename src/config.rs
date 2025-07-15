use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub db_path: PathBuf,
    pub base_dir: PathBuf,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let base_dir = env::var("BASE_DIR").context("BASE_DIR environment variable not set")?;
        let db_path = env::var("DB_PATH").unwrap_or_else(|_| "backup-buddy.db".to_string());

        Ok(Self {
            db_path: PathBuf::from(db_path),
            base_dir: PathBuf::from(base_dir),
        })
    }
}
