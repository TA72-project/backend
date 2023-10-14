use diesel::{
    migration::MigrationVersion,
    r2d2::{self},
    PgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub fn create_pool() -> DbPool {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env var should be set");

    let manager = r2d2::ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Unable to connect to database")
}

pub fn run_migrations<DB: diesel::backend::Backend>(
    con: &mut impl MigrationHarness<DB>,
) -> diesel::migration::Result<Vec<MigrationVersion>> {
    con.run_pending_migrations(MIGRATIONS)
}
