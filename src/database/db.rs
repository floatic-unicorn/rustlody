use sqlx::{Pool, MySql, Error, MySqlPool};
use sqlx::mysql::MySqlConnectOptions;


async fn get_db_pool() -> Result<Pool<MySql>, Error> {
    let options = MySqlConnectOptions::new()
        .username("root")
        .password("root")
        .host("127.0.0.1")
        .port(3307)
        .database("lxpantos");
    return MySqlPool::connect_with(options).await;
}

async fn insert_test_user(pool: &Pool<MySql>) {
    sqlx::raw_sql(
        "INSERT INTO account (account_id, type, entity_id, role, email, password, created_at, updated_at) \
        VALUES (1, 'FLODY_CONSOLE', null, 'ROLE_FLODY_CONSOLE', 'test@floactic.io', 'password', now(), now())"
    )
    .execute(pool)
    .await
    .unwrap();
}

pub async fn setup() {
    let pool = get_db_pool().await.unwrap();
    insert_test_user(&pool).await;
}
