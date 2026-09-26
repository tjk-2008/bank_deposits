mod models;

use models::{Account, Client, Deposit};

fn main() {
    // 1. Создаём вид вклада
    let deposit = Deposit::create_new("Срочный вклад «Выгодный»".to_string(), 12, 7.5);

    // 2. Создаём клиента
    let client = Client::create_new(
        "Иванов Иван Иванович".to_string(),
        "4509 123456".to_string(),
        "г. Москва, ул. Тверская, д. 1".to_string(),
        "+7-900-123-45-67".to_string(),
    );

    // 3. Открываем счёт
    let account = Account::create_new(&client, &deposit, 100_000.0);

    println!(
        "Вклад:   {} ({} мес., {}%)",
        deposit.name, deposit.term_months, deposit.rate_percent
    );
    println!("Клиент:  {}", client.full_name);
    println!("Счёт:    {} на сумму {} руб.", account.id, account.amount);
}

use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Проверка: считаем количество вкладов
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM deposits")
        .fetch_one(&pool)
        .await?;

    println!("Вкладов в БД: {}", count.0);

    Ok(())
}
