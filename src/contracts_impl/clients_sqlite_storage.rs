use async_sqlite::{JournalMode, Pool, PoolBuilder};
use rusqlite::{params_from_iter, types::Value, OptionalExtension};
use std::path::PathBuf;
use uuid::Uuid;

use crate::contracts::ClientsStorage;
use crate::entities::Client;

pub struct ClientsSqliteStorage {
    connection: Pool,
}

impl ClientsSqliteStorage {
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

impl ClientsStorage for ClientsSqliteStorage {
    async fn add(&self, client: Client) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            INSERT INTO clients
                (id, full_name, passport_number, address, phone,
                 created_at, updated_at, deleted_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
        ";

        let parameters: Vec<Value> = vec![
            client.id.into(),
            client.full_name.into(),
            client.passport_number.into(),
            client.address.into(),
            client.phone.into(),
            client.created_at.to_string().into(),
            client.updated_at.map(|d| d.to_string()).into(),
            client.deleted_at.map(|d| d.to_string()).into(),
        ];

        self.connection
            .conn(move |connection| {
                connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(())
            })
            .await?;

        Ok(())
    }

    async fn get(&self, uuid: Uuid) -> Result<Option<Client>, Box<dyn std::error::Error>> {
        const QUERY: &str = "
            SELECT id, full_name, passport_number, address, phone,
                   created_at, updated_at, deleted_at
            FROM clients
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
                            row.get::<_, String>(2)?,
                            row.get::<_, String>(3)?,
                            row.get::<_, String>(4)?,
                            row.get::<_, String>(5)?,
                            row.get::<_, Option<String>>(6)?,
                            row.get::<_, Option<String>>(7)?,
                        ))
                    })
                    .optional()?;
                Ok(row)
            })
            .await?;

        let client = row.map(
            |(id, full_name, passport_number, address, phone, _c, _u, _d)| Client {
                id,
                full_name,
                passport_number,
                address,
                phone,
                created_at: time::UtcDateTime::now(),
                updated_at: None,
                deleted_at: None,
            },
        );

        Ok(client)
    }

    async fn remove(&self, uuid: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "DELETE FROM clients WHERE id = ?1";
        let parameters: Vec<Value> = vec![uuid.into()];

        self.connection
            .conn(move |connection| {
                connection.execute(QUERY, params_from_iter(parameters))?;
                Ok(())
            })
            .await?;

        Ok(())
    }

    async fn update(&self, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
        const QUERY: &str = "
            UPDATE clients
            SET full_name = ?1, passport_number = ?2, address = ?3, phone = ?4,
                updated_at = ?5, deleted_at = ?6
            WHERE id = ?7
        ";

        let parameters: Vec<Value> = vec![
            client.full_name.clone().into(),
            client.passport_number.clone().into(),
            client.address.clone().into(),
            client.phone.clone().into(),
            client.updated_at.map(|d| d.to_string()).into(),
            client.deleted_at.map(|d| d.to_string()).into(),
            client.id.into(),
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
