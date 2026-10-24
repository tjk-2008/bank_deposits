use rusqlite::{params_from_iter, types::Value, OptionalExtension};
use uuid::Uuid;

use crate::contracts::ClientsStorage;
use crate::contracts_impl::datetime;
use crate::entities::Client;

pub struct ClientsSqliteStorage {
    connection: async_sqlite::Client,
}

impl ClientsSqliteStorage {
    /// Создаёт хранилище поверх соединения, выданного пулом.
    pub fn from_connection(connection: async_sqlite::Client) -> Self {
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
            datetime::format(client.created_at).into(),
            datetime::format_opt(client.updated_at).into(),
            datetime::format_opt(client.deleted_at).into(),
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

        let Some((id, full_name, passport_number, address, phone, created_at, updated_at, deleted_at)) =
            row
        else {
            return Ok(None);
        };

        Ok(Some(Client {
            id,
            full_name,
            passport_number,
            address,
            phone,
            created_at: datetime::parse(&created_at)?,
            updated_at: datetime::parse_opt(updated_at.as_deref())?,
            deleted_at: datetime::parse_opt(deleted_at.as_deref())?,
        }))
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
            datetime::format_opt(client.updated_at).into(),
            datetime::format_opt(client.deleted_at).into(),
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
