use serde::{Deserialize, Serialize};

// Вклад
#[derive(Deserialize)]
pub struct AddDepositPayload {
    pub name: String,
    pub term_months: i32,
    pub rate_percent: f32,
}

#[derive(Deserialize)]
pub struct UpdateDepositPayload {
    pub name: String,
    pub term_months: i32,
    pub rate_percent: f32,
}

#[derive(Serialize)]
pub struct DepositResponse {
    pub id: String,
    pub name: String,
    pub term_months: i32,
    pub rate_percent: f32,
}

// Клиент
#[derive(Deserialize)]
pub struct AddClientPayload {
    pub full_name: String,
    pub passport_number: String,
    pub address: String,
    pub phone: String,
}

#[derive(Deserialize)]
pub struct UpdateClientPayload {
    pub full_name: String,
    pub passport_number: String,
    pub address: String,
    pub phone: String,
}

#[derive(Serialize)]
pub struct ClientResponse {
    pub id: String,
    pub full_name: String,
    pub passport_number: String,
    pub address: String,
    pub phone: String,
}

// Счёт
#[derive(Deserialize)]
pub struct AddAccountPayload {
    pub client_id: String,
    pub deposit_id: String,
    pub amount: f32,
}

#[derive(Deserialize)]
pub struct UpdateAccountPayload {
    pub client_id: String,
    pub deposit_id: String,
    pub amount: f32,
}

#[derive(Serialize)]
pub struct AccountResponse {
    pub id: String,
    pub client_id: String,
    pub deposit_id: String,
    pub opened_at: String,
    pub closed_at: Option<String>,
    pub amount: f32,
}

// Общий ответ об ошибке
#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}
