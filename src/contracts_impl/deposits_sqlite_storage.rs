use async_sqlite::{JournalMode, Pool, PoolBuilder};
use rusqlite::{params_from_iter, types::Value, OptionalExtension};
use std::path::PathBuf;
use uuid::Uuid;

use crate::contracts::DepositsStorage;
use crate::contracts_impl::datetime;
use crate::entities::Deposit;

pub struct DepositsSqliteStorage {
    connection: Pool,
}

impl DepositsSqliteStorage {
    // Используется при самостоятельном создании хранилища; HTTP-слой
    // берёт общее соединение через `from_pool`.
    #[allow(dead_code)]
    pub async fn new(database_url: &str) -> Self {
        let path_buf = PathBuf::from(database_url);
        let builder = PoolBuilder::new()
            .path(path_buf)
            .journal_mode(JournalMode::Wal)
            .num_conns(10);
        let connection = builder.open().await.expect("Database creation error");
        Self { connection }
    }

    pub fn from_pool(connection: Pool) -> Self {
        Self { connection }
    }
}

impl DepositsStorage for DepositsSqliteStorage {
    async fn add(&self, deposit: Deposit) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            INSERT INTO deposits
                (id, name, term_months, rate_percent, created_at, updated_at, deleted_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        ";

        let parameters: Vec<Value> = vec![
            deposit.id.into(),
            deposit.name.into(),
            deposit.term_months.into(),
            deposit.rate_percent.into(),
            datetime::format(deposit.created_at).into(),
            datetime::format_opt(deposit.updated_at).into(),
            datetime::format_opt(deposit.deleted_at).into(),
        ];

        self.connection
            .conn(move |connection| {
                connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(())
            })
            .await?;

        Ok(())
    }

    async fn get(&self, uuid: Uuid) -> Result<Option<Deposit>, Box<dyn std::error::Error>> {
        const QUERY: &str = "
            SELECT id, name, term_months, rate_percent, created_at, updated_at, deleted_at
            FROM deposits
            WHERE id = ?1
        ";

        let parameters: Vec<Value> = vec![uuid.into()];

        let row = self
            .connection
            .conn(move |connection| {
                let row = connection
                    .query_row(QUERY, params_from_iter(parameters), |row| {
                        Ok((
                            row.get::<_, Uuid>(0)?,
                            row.get::<_, String>(1)?,
                            row.get::<_, i32>(2)?,
                            row.get::<_, f32>(3)?,
                            row.get::<_, String>(4)?,
                            row.get::<_, Option<String>>(5)?,
                            row.get::<_, Option<String>>(6)?,
                        ))
                    })
                    .optional()?;
                Ok(row)
            })
            .await?;

        let Some((id, name, term_months, rate_percent, created_at, updated_at, deleted_at)) = row
        else {
            return Ok(None);
        };

        Ok(Some(Deposit {
            id,
            name,
            term_months,
            rate_percent,
            created_at: datetime::parse(&created_at)?,
            updated_at: datetime::parse_opt(updated_at.as_deref())?,
            deleted_at: datetime::parse_opt(deleted_at.as_deref())?,
        }))
    }

    async fn remove(&self, uuid: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "DELETE FROM deposits WHERE id = ?1";
        let parameters: Vec<Value> = vec![uuid.into()];

        self.connection
            .conn(move |connection| {
                connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(())
            })
            .await?;

        Ok(())
    }

    async fn update(&self, deposit: &Deposit) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            UPDATE deposits
            SET name = ?1, term_months = ?2, rate_percent = ?3,
                updated_at = ?4, deleted_at = ?5
            WHERE id = ?6
        ";

        let parameters: Vec<Value> = vec![
            deposit.name.clone().into(),
            deposit.term_months.into(),
            deposit.rate_percent.into(),
            datetime::format_opt(deposit.updated_at).into(),
            datetime::format_opt(deposit.deleted_at).into(),
            deposit.id.into(),
        ];

        self.connection
            .conn(move |connection| {
                connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(())
            })
            .await?;

        Ok(())
    }
}
