use async_sqlite::{JournalMode, Pool, PoolBuilder};
use rusqlite::{params_from_iter, types::Value, OptionalExtension};
use std::path::PathBuf;
use uuid::Uuid;

use crate::contracts::AccountsStorage;
use crate::contracts_impl::datetime;
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
            datetime::format(account.opened_at).into(),
            datetime::format_opt(account.closed_at).into(),
            account.amount.into(),
            datetime::format(account.created_at).into(),
            datetime::format_opt(account.updated_at).into(),
            datetime::format_opt(account.deleted_at).into(),
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

        let Some((id, client_id, deposit_id, opened_at, closed_at, amount, created_at, updated_at, deleted_at)) =
            row
        else {
            return Ok(None);
        };

        Ok(Some(Account {
            id,
            client_id,
            deposit_id,
            opened_at: datetime::parse(&opened_at)?,
            closed_at: datetime::parse_opt(closed_at.as_deref())?,
            amount,
            created_at: datetime::parse(&created_at)?,
            updated_at: datetime::parse_opt(updated_at.as_deref())?,
            deleted_at: datetime::parse_opt(deleted_at.as_deref())?,
        }))
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
            datetime::format(account.opened_at).into(),
            datetime::format_opt(account.closed_at).into(),
            account.amount.into(),
            datetime::format_opt(account.updated_at).into(),
            datetime::format_opt(account.deleted_at).into(),
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
