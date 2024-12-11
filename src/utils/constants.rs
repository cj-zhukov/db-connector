use std::{env as std_env, sync::LazyLock};

use dotenvy::dotenv;

pub const PG_TABLE_NAME: &str = "users";
pub const PG_MAX_CONS: u32 = 10;
pub const SQLITE_TABLE_NAME: &str = "users";
pub const SQLITE_MAX_CONS: u32 = 10;

pub static PG_DATABASE_URL: LazyLock<String> = LazyLock::new(|| {
    dotenv().ok();
    let secret = std_env::var(env::PG_DATABASE_URL_ENV_VAR)
        .expect("DATABASE_URL_ENV_VAR must be set.");
    if secret.is_empty() {
        panic!("DATABASE_URL_ENV_VAR must not be empty.");
    }
    secret
});

pub static SQLITE_DATABASE_URL: LazyLock<String> = LazyLock::new(|| {
    dotenv().ok();
    let secret = std_env::var(env::SQLITE_DATABASE_URL_ENV_VAR)
        .expect("SQLITE_DATABASE_URL_ENV_VAR must be set.");
    if secret.is_empty() {
        panic!("SQLITE_DATABASE_URL_ENV_VAR must not be empty.");
    }
    secret
});

pub mod env {
    pub const PG_DATABASE_URL_ENV_VAR: &str = "PG_DATABASE_URL";
    pub const SQLITE_DATABASE_URL_ENV_VAR: &str = "SQLITE_DATABASE_URL";
}