use ohkami::claw::Path;
use ohkami::fang::Context;
use ohkami::{Json, Response};
use time::UtcDateTime;
use uuid::Uuid;

use crate::context::RequestContext;
use crate::contracts::DepositsStorage;
use crate::contracts_impl::DepositsSqliteStorage;
use crate::entities::Deposit;

use super::payloads::{AddDepositPayload, DepositResponse, ErrorResponse, UpdateDepositPayload};

pub async fn add_deposit(
    Json(payload): Json<AddDepositPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    if payload.name.trim().is_empty() {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "name must not be empty".to_string(),
        });
    }
    if payload.term_months <= 0 {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "term_months must be positive".to_string(),
        });
    }
    if payload.rate_percent < 0.0 {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "rate_percent must not be negative".to_string(),
        });
    }

    let deposit = Deposit::create_new(payload.name, payload.term_months, payload.rate_percent);
    let response_payload = DepositResponse {
        id: deposit.id.to_string(),
        name: deposit.name.clone(),
        term_months: deposit.term_months,
        rate_percent: deposit.rate_percent,
    };

    let storage = DepositsSqliteStorage::from_pool((*context.connection_pool).clone());
    match storage.add(deposit).await {
        Ok(()) => Response::Created().with_json(response_payload),
        Err(error) => {
            eprintln!("Ошибка добавления вклада: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}

pub async fn get_deposit(
    Path(id): Path<&str>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid deposit id".to_string(),
        });
    };

    let storage = DepositsSqliteStorage::from_pool((*context.connection_pool).clone());
    match storage.get(id).await {
        Ok(Some(deposit)) => Response::OK().with_json(DepositResponse {
            id: deposit.id.to_string(),
            name: deposit.name.clone(),
            term_months: deposit.term_months,
            rate_percent: deposit.rate_percent,
        }),
        Ok(None) => Response::NotFound().with_json(ErrorResponse {
            message: "deposit not found".to_string(),
        }),
        Err(error) => {
            eprintln!("Ошибка чтения вклада: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}

pub async fn update_deposit(
    Path(id): Path<&str>,
    Json(payload): Json<UpdateDepositPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid deposit id".to_string(),
        });
    };
    if payload.name.trim().is_empty() {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "name must not be empty".to_string(),
        });
    }
    if payload.term_months <= 0 {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "term_months must be positive".to_string(),
        });
    }
    if payload.rate_percent < 0.0 {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "rate_percent must not be negative".to_string(),
        });
    }

    let storage = DepositsSqliteStorage::from_pool((*context.connection_pool).clone());
    let existing = match storage.get(id).await {
        Ok(Some(deposit)) => deposit,
        Ok(None) => {
            return Response::NotFound().with_json(ErrorResponse {
                message: "deposit not found".to_string(),
            })
        }
        Err(error) => {
            eprintln!("Ошибка чтения вклада: {error}");
            return Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            });
        }
    };

    let updated = Deposit {
        id: existing.id,
        name: payload.name,
        term_months: payload.term_months,
        rate_percent: payload.rate_percent,
        created_at: existing.created_at,
        updated_at: Some(UtcDateTime::now()),
        deleted_at: existing.deleted_at,
    };

    match storage.update(&updated).await {
        Ok(()) => Response::OK().with_json(DepositResponse {
            id: updated.id.to_string(),
            name: updated.name.clone(),
            term_months: updated.term_months,
            rate_percent: updated.rate_percent,
        }),
        Err(error) => {
            eprintln!("Ошибка обновления вклада: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}

pub async fn remove_deposit(
    Path(id): Path<&str>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid deposit id".to_string(),
        });
    };

    let storage = DepositsSqliteStorage::from_pool((*context.connection_pool).clone());
    match storage.get(id).await {
        Ok(Some(_)) => {}
        Ok(None) => {
            return Response::NotFound().with_json(ErrorResponse {
                message: "deposit not found".to_string(),
            })
        }
        Err(error) => {
            eprintln!("Ошибка чтения вклада: {error}");
            return Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            });
        }
    }

    match storage.remove(id).await {
        Ok(()) => Response::OK().with_json(serde_json::json!({ "removed": true })),
        Err(error) => {
            eprintln!("Ошибка удаления вклада: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}
