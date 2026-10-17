use std::sync::Arc;

use async_sqlite::Pool;

/// Общий контекст запроса: пул соединений, доступный всем обработчикам.
/// Обёрнут в `Arc`, потому что Ohkami требует `Clone` для контекста,
/// а клонирование `Pool` дешёвое — копируется только указатель.
#[derive(Clone)]
pub struct RequestContext {
    pub connection_pool: Arc<Pool>,
}

impl RequestContext {
    pub fn new(connection_pool: Arc<Pool>) -> Self {
        Self { connection_pool }
    }
}
