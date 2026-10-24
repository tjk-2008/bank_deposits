pub mod accounts;
pub mod clients;
pub mod deposits;
pub mod payloads;

use ohkami::Response;

use payloads::ErrorResponse;

/// Единый ответ на ошибку работы с базой данных.
pub(crate) fn database_error() -> Response {
    Response::InternalServerError().with_json(ErrorResponse {
        message: "database error".to_string(),
    })
}
