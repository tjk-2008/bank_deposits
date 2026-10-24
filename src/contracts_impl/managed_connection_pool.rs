//! Управляемый пул соединений с SQLite.
//!
//! Библиотечный `async_sqlite::Pool` выдаёт произвольное свободное соединение
//! на каждый вызов `conn(...)`, поэтому запросы одной бизнес-операции могут
//! попасть на разные соединения. Здесь пул устроен иначе: соединение
//! забирается один раз в начале операции и возвращается только после того,
//! как все запросы выполнены. Это позволяет выполнять их в одной транзакции.

use std::fmt;

use async_sqlite::{Client, ClientBuilder, JournalMode};
use smol::channel::{Receiver, RecvError, SendError, Sender};

/// Канал, через который пул выдаёт соединения и принимает их обратно.
type ManagedPool = (Sender<Client>, Receiver<Client>);

pub struct ManagedConnectionPool {
    database_path: String,
    max_connections_count: usize,
    pool: ManagedPool,
}

impl fmt::Debug for ManagedConnectionPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ManagedConnectionPool")
            .field("database_path", &self.database_path)
            .field("max_connections_count", &self.max_connections_count)
            .finish()
    }
}

impl ManagedConnectionPool {
    /// Создаёт пул и сразу наполняет его соединениями.
    pub async fn initialize(database_path: String, max_connections_count: usize) -> Self {
        assert!(
            max_connections_count > 0,
            "Размер пула соединений должен быть больше нуля."
        );

        let (sender, receiver) = smol::channel::bounded(max_connections_count);

        for _ in 0..max_connections_count {
            let connection = Self::create_connection(database_path.clone()).await;
            sender
                .send(connection)
                .await
                .expect("Не удается наполнить пул соединений.");
        }

        Self {
            database_path,
            max_connections_count,
            pool: (sender, receiver),
        }
    }

    /// Открывает одно соединение с базой данных.
    async fn create_connection(database_path: String) -> Client {
        ClientBuilder::new()
            .path(database_path)
            .journal_mode(JournalMode::Wal)
            .open()
            .await
            .expect("Не удается открыть соединение.")
    }

    /// Забирает соединение из пула.
    pub async fn acquire_connection(&self) -> Result<Client, RecvError> {
        self.pool.1.recv().await
    }

    /// Возвращает соединение обратно в пул.
    ///
    /// В обработчиках используется [`PooledConnection`], который возвращает
    /// соединение автоматически; этот метод оставлен для явного сценария.
    #[allow(dead_code)]
    pub async fn return_connection(&self, connection: Client) -> Result<(), SendError<Client>> {
        self.pool.0.send(connection).await
    }

    /// Забирает соединение и оборачивает его в [`PooledConnection`],
    /// который вернёт соединение в пул при разрушении.
    pub async fn acquire(&self) -> Result<PooledConnection, RecvError> {
        let connection = self.acquire_connection().await?;
        Ok(PooledConnection {
            connection: Some(connection),
            sender: self.pool.0.clone(),
        })
    }
}

/// Соединение, выданное пулом на время одной бизнес-операции.
///
/// Возврат в пул происходит в [`Drop`], поэтому соединение не потеряется
/// даже при раннем `return` из обработчика.
pub struct PooledConnection {
    connection: Option<Client>,
    sender: Sender<Client>,
}

impl PooledConnection {
    pub fn client(&self) -> &Client {
        self.connection
            .as_ref()
            .expect("соединение уже возвращено в пул")
    }
}

impl Drop for PooledConnection {
    fn drop(&mut self) {
        if let Some(connection) = self.connection.take() {
            // Ёмкость канала равна размеру пула, а это соединение было из него
            // изъято, поэтому свободное место гарантированно есть.
            let _ = self.sender.try_send(connection);
        }
    }
}
