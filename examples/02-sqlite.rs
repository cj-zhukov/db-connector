use std::sync::Arc;

use color_eyre::Result;
use db_connector::domain::Database;
use secrecy::ExposeSecret;
use tokio::sync::RwLock;

use db_connector::service::SqliteDb;
use db_connector::execute_query;
use db_connector::utils::{SQLITE_DATABASE_URL, SQLITE_TABLE_NAME, SQLITE_MAX_CONS};

#[tokio::main]
async fn main() -> Result<()> {  
    color_eyre::install()?;
    
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