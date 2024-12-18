use async_trait::async_trait;
use secrecy::Secret;
use sqlx::migrate::Migrator;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::domain::Database;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations/postgres");

pub struct PostgresDbBuilder {
    url: String,
    max_cons: u32,
}

impl Default for PostgresDbBuilder {
    fn default() -> Self {
        PostgresDbBuilder { url: String::default(), max_cons: 10 }
    }
}

impl PostgresDbBuilder {
    pub fn with_url(self, url: &str) -> Self {
        Self { url: url.to_string(), max_cons: self.max_cons }
    }

    pub fn with_max_cons(self, max_cons: u32) -> Self {
        Self { url: self.url, max_cons }
    }

    pub async fn build(self) -> Result<PostgresDb, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(self.max_cons)
            .connect(&self.url)
            .await?;

        Ok(PostgresDb { pool, url: Secret::new(self.url) })
    }
}

#[derive(Debug)]
pub struct PostgresDb {
    pool: Pool<Postgres>,
    pub url: Secret<String>, 
}

impl AsRef<Pool<Postgres>> for PostgresDb {
    fn as_ref(&self) -> &Pool<Postgres> {
        &self.pool
    }
}

impl PostgresDb {
    pub fn builder() -> PostgresDbBuilder {
        PostgresDbBuilder::default()
    }
}

#[async_trait]
impl Database for PostgresDb {
    async fn execute_query(&self, query: &str) -> Result<(), sqlx::Error> {
        let rows = sqlx::query(query)
            .fetch_all(self.as_ref())
            .await?;

        for row in rows {
            println!("{:?}", row);
        }

        Ok(())
    }

    async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        MIGRATOR
            .run(self.as_ref())
            .await?;

        println!("run migrations for pg server");

        Ok(())
    }
}