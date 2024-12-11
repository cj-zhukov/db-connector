use async_trait::async_trait;

#[async_trait]
pub trait Database {
    async fn run_migrations(&self) -> Result<(), sqlx::Error>;
    async fn execute_query(&self, query: &str) -> Result<(), sqlx::Error>;
}