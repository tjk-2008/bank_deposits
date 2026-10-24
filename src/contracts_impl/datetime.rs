//! Единый формат хранения дат в SQLite.
//!
//! Даты пишутся в RFC 3339 и читаются обратно без потери значения,
//! поэтому `GET` возвращает ровно то, что было сохранено.

use time::{format_description::well_known::Rfc3339, UtcDateTime};

/// Сериализует дату для записи в столбец `TEXT`.
pub fn format(dt: UtcDateTime) -> String {
    dt.format(&Rfc3339)
        .unwrap_or_else(|_| dt.to_string())
}

/// Сериализует необязательную дату (`None` записывается как `NULL`).
pub fn format_opt(dt: Option<UtcDateTime>) -> Option<String> {
    dt.map(format)
}

/// Читает дату из столбца `TEXT`.
pub fn parse(value: &str) -> Result<UtcDateTime, time::error::Parse> {
    UtcDateTime::parse(value, &Rfc3339)
}

/// Читает необязательную дату из столбца `TEXT`.
pub fn parse_opt(value: Option<&str>) -> Result<Option<UtcDateTime>, time::error::Parse> {
    value.map(parse).transpose()
}
