use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use crate::env::env::ENV;

pub type Database = r2d2::Pool<ConnectionManager<PgConnection>>;

pub type DBConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn init() -> Database {
    let conn = ConnectionManager::<PgConnection>::new(ENV.database_url.clone());
    let pool = r2d2::Pool::builder()
        .max_size(40)
        .build(conn)
        .expect("Cannot build a pool");

    pool.get().unwrap().run_pending_migrations(MIGRATIONS).unwrap();

    pool
}