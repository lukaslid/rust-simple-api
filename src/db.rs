#![allow(unused_must_use, unused_imports)]
use diesel::{pg::PgConnection, r2d2::ConnectionManager, sql_query, RunQueryDsl};
use diesel_migrations::embed_migrations;
use dotenv::dotenv;
use r2d2::Pool;
use std::env;

embed_migrations!();

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_db_pool(db_url: &String) -> PostgresPool {
    dotenv().ok();
    let url = env::var(db_url).expect("DB URL is not set.");
    let migr = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(migr)
        .expect("Could not build connection pool.")
}

#[cfg(not(test))]
pub fn init_database(db_url: &String) -> PostgresPool {
    let pool = get_db_pool(db_url);

    embedded_migrations::run(&pool.get().expect("Could not establish a connection."));
    pool
}

#[cfg(test)]
pub fn init_database(db_url: &String) -> PostgresPool {
    let pool = get_db_pool(db_url);
    let conn = pool.get().expect("Could not establish a connection.");

    sql_query(r#"DROP TABLE IF EXISTS "user";"#).execute(&conn);
    sql_query(r#"DROP TABLE IF EXISTS "__diesel_schema_migrations";"#).execute(&conn);

    embedded_migrations::run(&conn);
    pool
}
