use ohkami::claw::Path;
use ohkami::fang::Context;
use ohkami::{Json, Response};
use time::UtcDateTime;
use uuid::Uuid;

use crate::context::RequestContext;
use crate::contracts::{AccountsStorage, ClientsStorage, DepositsStorage};
use crate::contracts_impl::{AccountsSqliteStorage, ClientsSqliteStorage, DepositsSqliteStorage};
use crate::entities::Account;

use super::database_error;
use super::payloads::{AccountResponse, AddAccountPayload, ErrorResponse, UpdateAccountPayload};

fn to_response(account: &Account) -> AccountResponse {
    AccountResponse {
        id: account.id.to_string(),
        client_id: account.client_id.to_string(),
        deposit_id: account.deposit_id.to_string(),
        opened_at: account.opened_at.to_string(),
        closed_at: account.closed_at.map(|d| d.to_string()),
        amount: account.amount,
    }
}

pub async fn add_account(
    Json(payload): Json<AddAccountPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(client_id) = Uuid::parse_str(&payload.client_id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid client_id".to_string(),
        });
    };
    let Ok(deposit_id) = Uuid::parse_str(&payload.deposit_id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid deposit_id".to_string(),
        });
    };
    if payload.amount <= 0.0 {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "amount must be positive".to_string(),
        });
    }

    let Ok(connection) = context.connection_pool.acquire().await else {
        return database_error();
    };
    let clients_storage = ClientsSqliteStorage::from_connection(connection.client().clone());
    let deposits_storage = DepositsSqliteStorage::from_connection(connection.client().clone());
    let accounts_storage = AccountsSqliteStorage::from_connection(connection.client().clone());

    let client = match clients_storage.get(client_id).await {
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
    let deposit = match deposits_storage.get(deposit_id).await {
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

    let account = Account::create_new(&client, &deposit, payload.amount);
    let response_payload = to_response(&account);

    match accounts_storage.add(account).await {
        Ok(()) => Response::Created().with_json(response_payload),
        Err(error) => {
            eprintln!("Ошибка добавления счёта: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}

pub async fn get_account(
    Path(id): Path<&str>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid account id".to_string(),
        });
    };

    let Ok(connection) = context.connection_pool.acquire().await else {
        return database_error();
    };
    let storage = AccountsSqliteStorage::from_connection(connection.client().clone());
    match storage.get(id).await {
        Ok(Some(account)) => Response::OK().with_json(to_response(&account)),
        Ok(None) => Response::NotFound().with_json(ErrorResponse {
            message: "account not found".to_string(),
        }),
        Err(error) => {
            eprintln!("Ошибка чтения счёта: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}

pub async fn update_account(
    Path(id): Path<&str>,
    Json(payload): Json<UpdateAccountPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid account id".to_string(),
        });
    };
    let Ok(client_id) = Uuid::parse_str(&payload.client_id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid client_id".to_string(),
        });
    };
    let Ok(deposit_id) = Uuid::parse_str(&payload.deposit_id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid deposit_id".to_string(),
        });
    };
    if payload.amount <= 0.0 {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "amount must be positive".to_string(),
        });
    }

    let Ok(connection) = context.connection_pool.acquire().await else {
        return database_error();
    };
    let storage = AccountsSqliteStorage::from_connection(connection.client().clone());
    let existing = match storage.get(id).await {
        Ok(Some(account)) => account,
        Ok(None) => {
            return Response::NotFound().with_json(ErrorResponse {
                message: "account not found".to_string(),
            })
        }
        Err(error) => {
            eprintln!("Ошибка чтения счёта: {error}");
            return Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            });
        }
    };

    let updated = Account {
        id: existing.id,
        client_id,
        deposit_id,
        opened_at: existing.opened_at,
        closed_at: existing.closed_at,
        amount: payload.amount,
        created_at: existing.created_at,
        updated_at: Some(UtcDateTime::now()),
        deleted_at: existing.deleted_at,
    };

    match storage.update(&updated).await {
        Ok(()) => Response::OK().with_json(to_response(&updated)),
        Err(error) => {
            eprintln!("Ошибка обновления счёта: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}

pub async fn remove_account(
    Path(id): Path<&str>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_json(ErrorResponse {
            message: "invalid account id".to_string(),
        });
    };

    let Ok(connection) = context.connection_pool.acquire().await else {
        return database_error();
    };
    let storage = AccountsSqliteStorage::from_connection(connection.client().clone());
    match storage.get(id).await {
        Ok(Some(_)) => {}
        Ok(None) => {
            return Response::NotFound().with_json(ErrorResponse {
                message: "account not found".to_string(),
            })
        }
        Err(error) => {
            eprintln!("Ошибка чтения счёта: {error}");
            return Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            });
        }
    }

    match storage.remove(id).await {
        Ok(()) => Response::OK().with_json(serde_json::json!({ "removed": true })),
        Err(error) => {
            eprintln!("Ошибка удаления счёта: {error}");
            Response::InternalServerError().with_json(ErrorResponse {
                message: "database error".to_string(),
            })
        }
    }
}
