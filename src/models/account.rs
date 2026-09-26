use time::UtcDateTime;
use uuid::Uuid;

use crate::models::client::Client;
use crate::models::deposit::Deposit;

#[doc = "Счёт клиента банка.
        Поля:
        - № счёта
        - код клиента
        - код вклада
        - дата открытия счёта
        - дата закрытия счёта
        - сумма вложенная (руб.)
        - дата создания
        - дата обновления
        - дата удаления"]
pub struct Account {
    pub(crate) id: Uuid,
    pub(crate) client_id: Uuid,
    pub(crate) deposit_id: Uuid,
    pub(crate) opened_at: UtcDateTime,
    pub(crate) closed_at: Option<UtcDateTime>,
    pub(crate) amount: f32,
    pub(crate) created_at: UtcDateTime,
    pub(crate) updated_at: Option<UtcDateTime>,
    pub(crate) deleted_at: Option<UtcDateTime>,
}

impl Account {
    pub fn create(
        id: Uuid,
        client: &Client,
        deposit: &Deposit,
        opened_at: UtcDateTime,
        closed_at: Option<UtcDateTime>,
        amount: f32,
        created_at: UtcDateTime,
        updated_at: Option<UtcDateTime>,
        deleted_at: Option<UtcDateTime>,
    ) -> Self {
        Self {
            id,
            client_id: client.id,
            deposit_id: deposit.id,
            opened_at,
            closed_at,
            amount,
            created_at,
            updated_at,
            deleted_at,
        }
    }

    pub fn create_new(client: &Client, deposit: &Deposit, amount: f32) -> Self {
        let id = Uuid::new_v4();
        let now = UtcDateTime::now();
        Self::create(id, client, deposit, now, None, amount, now, None, None)
    }
}
