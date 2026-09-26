use time::UtcDateTime;
use uuid::Uuid;

#[doc = "Клиент банка.
        Поля:
        - код клиента
        - Ф.И.О. клиента
        - номер паспорта (уникальный)
        - адрес
        - телефон (уникальный)
        - дата создания
        - дата обновления
        - дата удаления"]
pub struct Client {
    pub(crate) id: Uuid,
    pub(crate) full_name: String,
    pub(crate) passport_number: String,
    pub(crate) address: String,
    pub(crate) phone: String,
    pub(crate) created_at: UtcDateTime,
    pub(crate) updated_at: Option<UtcDateTime>,
    pub(crate) deleted_at: Option<UtcDateTime>,
}

impl Client {
    pub fn create(
        id: Uuid,
        full_name: String,
        passport_number: String,
        address: String,
        phone: String,
        created_at: UtcDateTime,
        updated_at: Option<UtcDateTime>,
        deleted_at: Option<UtcDateTime>,
    ) -> Self {
        Self {
            id,
            full_name,
            passport_number,
            address,
            phone,
            created_at,
            updated_at,
            deleted_at,
        }
    }

    pub fn create_new(
        full_name: String,
        passport_number: String,
        address: String,
        phone: String,
    ) -> Self {
        let id = Uuid::new_v4();
        let created_at = UtcDateTime::now();
        Self::create(
            id,
            full_name,
            passport_number,
            address,
            phone,
            created_at,
            None,
            None,
        )
    }
}
