use std::sync::Arc;

use tokio::sync::RwLock;

pub mod domain;
pub mod service;
pub mod utils;

use crate::domain::Database;

pub type DbConRef<T> = Arc<RwLock<DbConnector<T>>>;

pub struct DbConnector<T: Database + Send + Sync> {
    db: T,
}

impl<T: Database + Send + Sync> DbConnector<T> {
    pub fn new(db: T) -> Self {
        Self { db }
    }

    // pub async fn connect(&mut self) -> Result<(), sqlx::Error> {
    //     self.db.connect().await
    // }

    pub async fn query(&self, query: &str) -> Result<(), sqlx::Error> {
        self.db.execute_query(query).await
    }
}

pub async fn execute_query<T>(db_connector: DbConRef<T>, query: String) -> Result<(), sqlx::Error>
where
    T: Database + Send + Sync,
{
    println!("Executing query: {}", query);
    let lock = db_connector.write().await;
    lock.query(&query).await
}