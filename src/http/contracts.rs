//! Реэкспорт контрактов (трейтов) хранилищ для HTTP-слоя.
//!
//! Позволяет из обработчиков писать `http::contracts::DepositsStorage`
//! вместо `crate::contracts::DepositsStorage`.

pub use crate::contracts::*;
