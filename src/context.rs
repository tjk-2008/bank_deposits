use std::sync::Arc;

use crate::contracts_impl::ManagedConnectionPool;

/// Общий контекст запроса: управляемый пул соединений, доступный всем
/// обработчикам. Обёрнут в `Arc`, потому что Ohkami требует `Clone`
/// для контекста, а клонирование `Arc` дешёвое — копируется только указатель.
#[derive(Clone)]
pub struct RequestContext {
    pub connection_pool: Arc<ManagedConnectionPool>,
}

impl RequestContext {
    pub fn new(connection_pool: Arc<ManagedConnectionPool>) -> Self {
        Self { connection_pool }
    }
}
