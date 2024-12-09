use async_trait::async_trait;

#[async_trait]
pub trait Database {
    // async fn connect(&mut self) -> Result<(), sqlx::Error>;
    async fn execute_query(&self, query: &str) -> Result<(), sqlx::Error>;
}