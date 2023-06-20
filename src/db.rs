use std::env;

use sqlx::{postgres::{PgConnectOptions, PgPoolOptions}, PgPool};

use crate::downloader::Member;

pub async fn init_db() -> PgPool {
    let db_username = env::var("DB_USERNAME").unwrap_or_else(|_| "".to_owned());
    let db_password = env::var("DB_PASSWORD").unwrap_or_else(|_| "".to_owned());
    let db_host = env::var("DB_HOST").unwrap_or_else(|_| "".to_owned());
    let db_port = env::var("DB_PORT").unwrap_or_else(|_| "".to_owned());
    let db_name = env::var("DB_NAME").unwrap_or_else(|_| "".to_owned());
    
    let options = PgConnectOptions::new()
        .host(&db_host)
        .username(&db_username)
        .password(&db_password)
        .port(db_port.parse().unwrap())
        .database(&db_name);

    // Connect to a Postgres Database
    let pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_with(options)
        .await
        .unwrap();

    pool
}

pub async fn get_member(pool: &PgPool) -> Vec<Member> {
    let members = sqlx::query!(
        "SELECT * 
        FROM members"
    )
    .fetch_all(pool)
    .await
    .expect("Failed to store test user.");

    members
}

pub async fn store_member(member: Member, pool: &PgPool)  {
    sqlx::query!(
        "INSERT INTO members (id, name)
        VALUES ($1, $2)",
        members.id,
        members.name
    )
    .execute(pool)
    .await
    .expect("Failed to store test user.");

}