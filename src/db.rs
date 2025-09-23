use anyhow::Result;
use diesel::{
    pg::PgConnection,
    r2d2,
    r2d2::{ConnectionManager, Pool},
};
use log::info;
use once_cell::sync::OnceCell;

use diesel_migrations::{
    EmbeddedMigrations, MigrationHarness, embed_migrations,
};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

static DB_POOL: OnceCell<DbPool> = OnceCell::new();

pub fn init(database_url: &str) -> Result<()> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager)?;
    let mut conn = pool.get()?; // Get a single connection

    run_migrations(&mut conn)?;

    DB_POOL
        .set(pool)
        .map_err(|_| anyhow::anyhow!("DB_POOL already set"))?;
    Ok(())
}

fn run_migrations(conn: &mut PgConnection) -> anyhow::Result<()> {
    info!("Running database migrations");
    conn.run_pending_migrations(MIGRATIONS)
        .map(|_| ())
        .map_err(|e| anyhow::anyhow!("Migration error: {}", e))
}

pub fn get_pool() -> &'static DbPool {
    DB_POOL.get().expect("DB pool not initialized")
}
