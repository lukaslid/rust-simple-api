use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use r2d2::Pool;
use std::env;

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_db_pool(db_url: &String) -> PostgresPool {
    dotenv().ok();
    let url = env::var(db_url).expect("DB URL is not set.");
    let migr = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(migr)
        .expect("Could not build connection pool.")
}
