use ohkami::claw::Path;
use ohkami::fang::Context;
use ohkami::{Json, Response};
use time::UtcDateTime;
use uuid::Uuid;

use crate::context::RequestContext;
use crate::contracts::ClientsStorage;
use crate::contracts_impl::ClientsSqliteStorage;
use crate::entities::Client;

use super::payloads::{AddClientPayload, ClientResponse, ErrorResponse, UpdateClientPayload};

pub async fn add_client(
    Json(payload): Json<AddClientPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    if payload.full_name.trim().is_empty() {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "full_name must not be empty".to_string(),
        });
    }
    if payload.passport_number.trim().is_empty() {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "passport_number must not be empty".to_string(),
        });
    }
    if payload.phone.trim().is_empty() {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "phone must not be empty".to_string(),
        });
    }

    let client = Client::create_new(
        payload.full_name,
        payload.passport_number,
        payload.address,
        payload.phone,
    );
    let response_payload = ClientResponse {
        id: client.id.to_string(),
        full_name: client.full_name.clone(),
        passport_number: client.passport_number.clone(),
        address: client.address.clone(),
        phone: client.phone.clone(),
    };

    let storage = ClientsSqliteStorage::from_pool((*context.connection_pool).clone());
    match storage.add(client).await {
        Ok(()) => Response::Created().with_json(response_payload),
        Err(error) => {
            eprintln!("Ошибка добавления клиента: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}

pub async fn get_client(
    Path(id): Path<&str>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid client id".to_string(),
        });
    };

    let storage = ClientsSqliteStorage::from_pool((*context.connection_pool).clone());
    match storage.get(id).await {
        Ok(Some(client)) => Response::OK().with_json(ClientResponse {
            id: client.id.to_string(),
            full_name: client.full_name.clone(),
            passport_number: client.passport_number.clone(),
            address: client.address.clone(),
            phone: client.phone.clone(),
        }),
        Ok(None) => Response::NotFound().with_json(ErrorResponse {
            message: "client not found".to_string(),
        }),
        Err(error) => {
            eprintln!("Ошибка чтения клиента: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}

pub async fn update_client(
    Path(id): Path<&str>,
    Json(payload): Json<UpdateClientPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid client id".to_string(),
        });
    };
    if payload.full_name.trim().is_empty() {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "full_name must not be empty".to_string(),
        });
    }
    if payload.passport_number.trim().is_empty() {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "passport_number must not be empty".to_string(),
        });
    }
    if payload.phone.trim().is_empty() {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "phone must not be empty".to_string(),
        });
    }

    let storage = ClientsSqliteStorage::from_pool((*context.connection_pool).clone());
    let existing = match storage.get(id).await {
        Ok(Some(client)) => client,
        Ok(None) => {
            return Response::NotFound().with_json(ErrorResponse {
                message: "client not found".to_string(),
            })
        }
        Err(error) => {
            eprintln!("Ошибка чтения клиента: {error}");
            return Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            });
        }
    };

    let updated = Client {
        id: existing.id,
        full_name: payload.full_name,
        passport_number: payload.passport_number,
        address: payload.address,
        phone: payload.phone,
        created_at: existing.created_at,
        updated_at: Some(UtcDateTime::now()),
        deleted_at: existing.deleted_at,
    };

    match storage.update(&updated).await {
        Ok(()) => Response::OK().with_json(ClientResponse {
            id: updated.id.to_string(),
            full_name: updated.full_name.clone(),
            passport_number: updated.passport_number.clone(),
            address: updated.address.clone(),
            phone: updated.phone.clone(),
        }),
        Err(error) => {
            eprintln!("Ошибка обновления клиента: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}

pub async fn remove_client(
    Path(id): Path<&str>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid client id".to_string(),
        });
    };

    let storage = ClientsSqliteStorage::from_pool((*context.connection_pool).clone());
    match storage.get(id).await {
        Ok(Some(_)) => {}
        Ok(None) => {
            return Response::NotFound().with_json(ErrorResponse {
                message: "client not found".to_string(),
            })
        }
        Err(error) => {
            eprintln!("Ошибка чтения клиента: {error}");
            return Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            });
        }
    }

    match storage.remove(id).await {
        Ok(()) => Response::OK().with_json(serde_json::json!({ "removed": true })),
        Err(error) => {
            eprintln!("Ошибка удаления клиента: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}
