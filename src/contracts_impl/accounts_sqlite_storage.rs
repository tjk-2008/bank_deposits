use async_sqlite::{JournalMode, Pool, PoolBuilder};
use rusqlite::{params_from_iter, types::Value, OptionalExtension};
use std::path::PathBuf;
use uuid::Uuid;

use crate::contracts::AccountsStorage;
use crate::entities::Account;

pub struct AccountsSqliteStorage {
    connection: Pool,
}

impl AccountsSqliteStorage {
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

impl AccountsStorage for AccountsSqliteStorage {
    async fn add(&self, account: Account) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            INSERT INTO accounts
                (id, client_id, deposit_id, opened_at, closed_at, amount,
                 created_at, updated_at, deleted_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
        ";

        let parameters: Vec<Value> = vec![
            account.id.into(),
            account.client_id.into(),
            account.deposit_id.into(),
            account.opened_at.to_string().into(),
            account.closed_at.map(|d| d.to_string()).into(),
            account.amount.into(),
            account.created_at.to_string().into(),
            account.updated_at.map(|d| d.to_string()).into(),
            account.deleted_at.map(|d| d.to_string()).into(),
        ];

        self.connection
            .conn(move |connection| {
                connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(())
            })
            .await?;

        Ok(())
    }

    async fn get(&self, uuid: Uuid) -> Result<Option<Account>, Box<dyn std::error::Error>> {
        const QUERY: &str = "
            SELECT id, client_id, deposit_id, opened_at, closed_at, amount,
                   created_at, updated_at, deleted_at
            FROM accounts
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
                            row.get::<_, Uuid>(1)?,
                            row.get::<_, Uuid>(2)?,
                            row.get::<_, String>(3)?,
                            row.get::<_, Option<String>>(4)?,
                            row.get::<_, f32>(5)?,
                            row.get::<_, String>(6)?,
                            row.get::<_, Option<String>>(7)?,
                            row.get::<_, Option<String>>(8)?,
                        ))
                    })
                    .optional()?;
                Ok(row)
            })
            .await?;

        let account = row.map(
            |(id, client_id, deposit_id, _opened, _closed, amount, _c, _u, _d)| Account {
                id,
                client_id,
                deposit_id,
                opened_at: time::UtcDateTime::now(),
                closed_at: None,
                amount,
                created_at: time::UtcDateTime::now(),
                updated_at: None,
                deleted_at: None,
            },
        );

        Ok(account)
    }

    async fn remove(&self, uuid: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "DELETE FROM accounts WHERE id = ?1";
        let parameters: Vec<Value> = vec![uuid.into()];

        self.connection
            .conn(move |connection| {
                connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(())
            })
            .await?;

        Ok(())
    }

    async fn update(&self, account: &Account) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            UPDATE accounts
            SET client_id = ?1, deposit_id = ?2, opened_at = ?3,
                closed_at = ?4, amount = ?5,
                updated_at = ?6, deleted_at = ?7
            WHERE id = ?8
        ";

        let parameters: Vec<Value> = vec![
            account.client_id.into(),
            account.deposit_id.into(),
            account.opened_at.to_string().into(),
            account.closed_at.map(|d| d.to_string()).into(),
            account.amount.into(),
            account.updated_at.map(|d| d.to_string()).into(),
            account.deleted_at.map(|d| d.to_string()).into(),
            account.id.into(),
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
