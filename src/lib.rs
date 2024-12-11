use std::sync::Arc;

use tokio::sync::RwLock;

pub mod domain;
pub mod service;
pub mod utils;

use crate::domain::Database;

pub type DbRef<Database> = Arc<RwLock<Database>>;

pub async fn execute_query<T: Database>(db: DbRef<T>, query: String) -> Result<(), sqlx::Error> {
    println!("Executing query: {}", query);
    let db = db.read().await;
    db.execute_query(&query).await
}