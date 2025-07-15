use anyhow::Result;
use r2d2_sqlite::SqliteConnectionManager;
use std::path::Path;
use tracing::info;

pub mod dao;
pub mod models;

pub fn init_db(db_path: &Path) -> Result<r2d2::Pool<SqliteConnectionManager>> {
    let manager = SqliteConnectionManager::file(db_path);
    let pool = r2d2::Pool::new(manager)?;

    let conn = pool.get()?;

    let current_version: usize = conn
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .unwrap_or(0);

    let migrations = vec![
        include_str!("../../migrations/0001_initial_schema.sql"),
        // include_str!("../../migrations/0002_add_error_column.sql"),
        // include_str!("../../migrations/0003_add_timeout_column.sql"),
        // include_str!("../../migrations/0004_add_purge_after_column.sql"),
        // Add more migrations here
    ];

    for (version, sql) in migrations.iter().enumerate() {
        if current_version <= version {
            info!("Running migration {}", version);
            conn.execute_batch(&sql)?;
            conn.execute(&format!("PRAGMA user_version = {}", version + 1), [])?;
        }
    }

    Ok(pool)
}
