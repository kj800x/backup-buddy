use std::str::FromStr;

use anyhow::Result;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::named_params;
use uuid::Uuid;

use super::models::*;

use rusqlite::{types::Type, Error as SqliteError};

trait IntoSqliteError<T> {
    fn sqlite_err(self) -> Result<T, SqliteError>;
}

impl<T, E> IntoSqliteError<T> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn sqlite_err(self) -> Result<T, SqliteError> {
        self.map_err(|e| SqliteError::FromSqlConversionFailure(0, Type::Text, Box::new(e)))
    }
}

pub struct PolicyDao {
    pool: Pool<SqliteConnectionManager>,
}

impl PolicyDao {
    pub fn new(pool: Pool<SqliteConnectionManager>) -> Self {
        Self { pool }
    }

    fn parse_policy_row(row: &rusqlite::Row) -> Result<BackupPolicy, rusqlite::Error> {
        Ok(BackupPolicy {
            id: Uuid::parse_str(&row.get::<_, String>(0)?).sqlite_err()?,
            path: row.get(1)?,
            max_staleness: row.get(2)?,
            kind: PolicyKind::from_str(&row.get::<_, String>(3)?).sqlite_err()?,
            recursive: row.get(4)?,
        })
    }

    pub fn create_policy(&self, policy: &BackupPolicy) -> Result<()> {
        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;

        tx.execute(
            "INSERT INTO backup_policies
            (id, path, max_staleness, kind, recursive)
            VALUES
            (:id, :path, :max_staleness, :kind, :recursive)",
            named_params![
                ":id": policy.id.to_string(),
                ":path": policy.path.clone(),
                ":max_staleness": policy.max_staleness,
                ":kind": policy.kind.to_string(),
                ":recursive": policy.recursive,
            ],
        )?;

        tx.commit()?;
        Ok(())
    }

    pub fn update_policy(&self, policy: &BackupPolicy) -> Result<()> {
        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;

        tx.execute(
            "UPDATE backup_policies
            SET path = :path, max_staleness = :max_staleness, kind = :kind, recursive = :recursive
            WHERE id = :id",
            named_params![
                ":id": policy.id.to_string(),
                ":path": policy.path.clone(),
                ":max_staleness": policy.max_staleness,
                ":kind": policy.kind.to_string(),
                ":recursive": policy.recursive,
            ],
        )?;

        tx.commit()?;
        Ok(())
    }

    pub fn delete_policy(&self, id: &Uuid) -> Result<()> {
        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;
        tx.execute(
            "DELETE FROM backup_policies WHERE id = :id",
            named_params![":id": id.to_string()],
        )?;
        tx.commit()?;
        Ok(())
    }

    pub fn get_policy(&self, id: &Uuid) -> Result<Option<BackupPolicy>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(
            "SELECT id, path, max_staleness, kind, recursive FROM backup_policies WHERE id = :id",
        )?;
        let policy =
            stmt.query_row(named_params![":id": id.to_string()], Self::parse_policy_row)?;

        Ok(Some(policy))
    }

    pub fn get_policies(&self) -> Result<Vec<BackupPolicy>> {
        let conn = self.pool.get()?;

        let mut stmt =
            conn.prepare("SELECT id, path, max_staleness, kind, recursive FROM backup_policies")?;
        let policies = stmt.query_map(named_params![], Self::parse_policy_row)?;

        Ok(policies
            .collect::<Result<Vec<BackupPolicy>, rusqlite::Error>>()?
            .into_iter()
            .collect())
    }
}
