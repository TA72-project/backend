//! Contains everything needed to use a Postgres database.

use diesel::{
    migration::MigrationVersion,
    r2d2::{self},
    sql_function,
    sql_types::{Nullable, Text},
    PgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

/// Database pool type shorthand
pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

/// Creates the database connection pool from `DATABASE_URL` environment variable.
///
/// Only Postgres is supported.
/// It must have the format `postgres://<user>:<password>@<host>/<database>`
pub fn create_pool() -> DbPool {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env var should be set");

    let manager = r2d2::ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Unable to connect to database")
}

/// Run migrations in the provided connection.
///
/// Migrations are embed in the binary.
pub fn run_migrations<DB: diesel::backend::Backend>(
    con: &mut impl MigrationHarness<DB>,
) -> diesel::migration::Result<Vec<MigrationVersion>> {
    con.run_pending_migrations(MIGRATIONS)
}

sql_function!(
    /// See [the PostgreSQL crypt documentation](https://www.postgresql.org/docs/current/pgcrypto.html#PGCRYPTO-PASSWORD-HASHING-FUNCS-CRYPT)
    ///
    /// The parameters and return are not really `Nullable<Text>` but rather `Text`.
    /// This is done in order to be compatible the the `password` field in database which *is*
    /// nullable.
    fn crypt(password: Text, salt: Nullable<Text>) -> Nullable<Text>
);

sql_function!(
    /// See [the PostgreSQL gen_salt documentation](https://www.postgresql.org/docs/current/pgcrypto.html#PGCRYPTO-PASSWORD-HASHING-FUNCS-GEN-SALT)
    ///
    /// The return is not really a `Nullable<Text>` but rather a `Text`.
    /// This is done in order to be compatible the the `password` field in database which *is*
    /// nullable.
    fn gen_salt(r#type: Text) -> Nullable<Text>
);
