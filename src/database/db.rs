use sqlx::mysql::MySqlConnectOptions;
use sqlx::{Error, MySql, MySqlPool, Pool};

async fn get_db_pool() -> Result<Pool<MySql>, Error> {
    let options = MySqlConnectOptions::new()
        .username("root")
        .password("root")
        .host("127.0.0.1")
        .port(3307)
        .database("lxpantos");
    MySqlPool::connect_with(options).await
}

async fn clear_and_insert_test_data(pool: &Pool<MySql>) {
    let tables: [&str; 14] = [
        "warehouse",
        "account",
        "robot_in_progress_picking",
        "robot_in_progress_unloading",
        "robot_picking_queue",
        "robot_unloading_queue",
        "robot",
        "loading_station",
        "work",
        "work_group",
        "wave",
        "picking",
        "picking_job",
        "path",
    ];
    for table in tables {
        let statement = String::from("DELETE FROM ") + table;
        println!("[SETUP] clearing tables: {}", statement);
        sqlx::raw_sql(&statement).execute(pool).await.unwrap();
    }

    let insert_stmts: [&str; 5] = [
        "INSERT INTO account (account_id, type, entity_id, role, email, password, created_at, updated_at) \
        VALUES (1, 'FLODY_CONSOLE', 1, 'ROLE_FLODY_CONSOLE', 'test@floactic.io', 'password', now(), now())",

        "INSERT INTO warehouse(warehouse_id, name, prefix, created_at, updated_at) \
        VALUES (1, 'test-warehouse', 'test-', now(), now())",

        "INSERT INTO robot(robot_id, warehouse_id, name, online, created_at, updated_at) \
        VALUES (1, 1, 'test-robot', true, now(), now())",

        "INSERT INTO loading_station(loading_station_id, warehouse_id, name, location_code, created_at, updated_at) \
        VALUES (1, 1, 'test-loading-station', 'test-loading-station-location-code', now(), now())",

        "INSERT INTO path(path_id, warehouse_id, location_code, priority, created_at, updated_at) \
        VALUES (1, 1, 'locationCode-1', 1, now(), now())",
    ];
    for stmt in insert_stmts {
        println!("[SETUP] inserting setup data: {}", stmt);
        sqlx::raw_sql(&stmt).execute(pool).await.unwrap();
    }
}

pub async fn setup() {
    let pool = get_db_pool().await.unwrap();
    clear_and_insert_test_data(&pool).await;
}
