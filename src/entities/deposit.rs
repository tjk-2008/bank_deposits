use time::UtcDateTime;
use uuid::Uuid;

#[doc = "Вклад, предоставляемый банком.
        Поля:
        - код вклада
        - наименование вклада
        - срок хранения (месяцев)
        - ставка, % годовых
        - дата создания
        - дата обновления
        - дата удаления"]
pub struct Deposit {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) term_months: i32,
    pub(crate) rate_percent: f32,
    pub(crate) created_at: UtcDateTime,
    pub(crate) updated_at: Option<UtcDateTime>,
    pub(crate) deleted_at: Option<UtcDateTime>,
}

impl Deposit {
    pub fn create(
        id: Uuid,
        name: String,
        term_months: i32,
        rate_percent: f32,
        created_at: UtcDateTime,
        updated_at: Option<UtcDateTime>,
        deleted_at: Option<UtcDateTime>,
    ) -> Self {
        Self {
            id,
            name,
            term_months,
            rate_percent,
            created_at,
            updated_at,
            deleted_at,
        }
    }

    pub fn create_new(name: String, term_months: i32, rate_percent: f32) -> Self {
        let id = Uuid::new_v4();
        let created_at = UtcDateTime::now();
        Self::create(id, name, term_months, rate_percent, created_at, None, None)
    }
}
