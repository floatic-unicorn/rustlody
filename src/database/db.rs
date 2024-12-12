use sqlx::mysql::MySqlConnectOptions;
use sqlx::{Error, MySql, MySqlPool, Pool};

use crate::database::wave_parser::parse_and_get_location_codes;

async fn get_db_pool() -> Result<Pool<MySql>, Error> {
    let options = MySqlConnectOptions::new()
        .username("root")
        .password("root")
        .host("127.0.0.1")
        .port(3307)
        .database("lxpantos");
    MySqlPool::connect_with(options).await
}

async fn clear_and_insert_test_data(pool: &Pool<MySql>, file_path: &str) {
    let tables: [&str; 15] = [
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
        "unloading_station",
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

        "INSERT INTO unloading_station(unloading_station_id, warehouse_id, name, location_code, created_at, updated_at) \
        VALUES (1, 1, 'unloading-station-1', 'unloading-station-1-location-code', now(), now())",
    ];
    for stmt in insert_stmts {
        println!("[SETUP] inserting setup data: {}", stmt);
        sqlx::raw_sql(&stmt).execute(pool).await.unwrap();
    }

    let location_codes = parse_and_get_location_codes(file_path);
    let mut insert_location_code_values_stmt = "".to_string();
    for location_code in location_codes {
        let value = format!("(1, '{location_code}', 1, now(), now()),");
        insert_location_code_values_stmt = insert_location_code_values_stmt + &value;
    }
    let mut insert_location_code_stmt = String::from("INSERT INTO path (warehouse_id, location_code, priority, created_at, updated_at) VALUES ") + &insert_location_code_values_stmt;
    let size = insert_location_code_stmt.len();
    insert_location_code_stmt.replace_range(size-1..size, ";");
    println!("[SETUP] inserting setup data: {}", insert_location_code_stmt);
    sqlx::raw_sql(&insert_location_code_stmt).execute(pool).await.unwrap();
}

pub async fn setup(file_path: &str) {
    let pool = get_db_pool().await.unwrap();
    clear_and_insert_test_data(&pool, file_path).await;
}
