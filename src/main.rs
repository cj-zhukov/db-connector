use std::sync::Arc;

use color_eyre::Result;
use tokio::sync::RwLock;

use db_connector::service::PostgresDb;
use db_connector::{DbConnector, execute_query};
use db_connector::utils::{PG_DATABASE_URL, PG_MAX_CONS, PG_TABLE_NAME};

#[tokio::main]
async fn main() -> Result<()>  {  
    let pg = PostgresDb::builder()
        .with_url(&PG_DATABASE_URL)
        .with_max_cons(PG_MAX_CONS)
        .build()
        .await?;
    let db_con = DbConnector::new(pg);
    let db_con_ref = Arc::new(RwLock::new(db_con));

    let mut tasks = vec![];
    for _ in 0..3 {
        let db_con_ref = db_con_ref.clone();
        let query = format!("select * from {PG_TABLE_NAME}");
        tasks.push(tokio::spawn(    execute_query(db_con_ref, query)));
    }

    for task in tasks {
        task.await??;
    }

    Ok(())
}