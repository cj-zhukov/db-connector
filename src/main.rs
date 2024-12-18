use std::sync::Arc;

use color_eyre::Result;
use db_connector::domain::Database;
use secrecy::ExposeSecret;
use tokio::sync::RwLock;

use db_connector::service::{PostgresDb, SqliteDb};
use db_connector::execute_query;
use db_connector::utils::{PG_DATABASE_URL, PG_MAX_CONS, PG_TABLE_NAME, SQLITE_DATABASE_URL, SQLITE_TABLE_NAME, SQLITE_MAX_CONS};

#[tokio::main]
async fn main() -> Result<()> {  
    pg_exec().await?;
    sqlite_exec().await?;

    Ok(())
}

pub async fn pg_exec() -> Result<()> {
    let db = PostgresDb::builder()
        .with_url(PG_DATABASE_URL.expose_secret())
        .with_max_cons(PG_MAX_CONS)
        .build()
        .await?;
    db.run_migrations().await?;
    let db_ref = Arc::new(RwLock::new(db));

    let mut tasks = vec![];
    for _ in 0..3 {
        let db_ref = db_ref.clone();
        let query = format!("select * from {PG_TABLE_NAME}");
        tasks.push(tokio::spawn(    execute_query(db_ref, query)));
    }

    for task in tasks {
        task.await??;
    }

    Ok(())
}

pub async fn sqlite_exec() -> Result<()> {
    let db = SqliteDb::builder()
        .with_url(SQLITE_DATABASE_URL.expose_secret())
        .with_max_cons(SQLITE_MAX_CONS)
        .build()
        .await?;
    db.run_migrations().await?;
    let db_ref = Arc::new(RwLock::new(db));

    let mut tasks = vec![];
    for _ in 0..3 {
        let db_ref = db_ref.clone();
        let query = format!("select * from {SQLITE_TABLE_NAME}");
        tasks.push(tokio::spawn(    execute_query(db_ref, query)));
    }

    for task in tasks {
        task.await??;
    }

    Ok(())
}