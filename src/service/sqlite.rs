use async_trait::async_trait;
use secrecy::Secret;
use sqlx::{migrate::Migrator, sqlite::SqlitePoolOptions, Pool, Row, Sqlite};

use crate::domain::Database;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations/sqlite");

pub struct SqliteDbBuilder {
    url: String,
    max_cons: u32,
}

impl Default for SqliteDbBuilder {
    fn default() -> Self {
        SqliteDbBuilder { url: String::default(), max_cons: 10 }
    }
}

impl SqliteDbBuilder {
    pub fn with_url(self, url: &str) -> Self {
        Self { url: url.to_string(), max_cons: self.max_cons }
    }

    pub fn with_max_cons(self, max_cons: u32) -> Self {
        Self { url: self.url, max_cons }
    }

    pub async fn build(self) -> Result<SqliteDb, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(self.max_cons)
            .connect(&self.url)
            .await?;

        Ok(SqliteDb { pool, url: Secret::new(self.url) })
    }
}

#[derive(Debug)]
pub struct SqliteDb {
    pool: Pool<Sqlite>,
    pub url: Secret<String>,
}

impl AsRef<Pool<Sqlite>> for SqliteDb {
    fn as_ref(&self) -> &Pool<Sqlite> {
        &self.pool
    }
}

impl SqliteDb {
    pub fn builder() -> SqliteDbBuilder {
        SqliteDbBuilder::default()
    }
}

#[async_trait]
impl Database for SqliteDb {
    async fn execute_query(&self, query: &str) -> Result<(), sqlx::Error> {
        let rows = sqlx::query(query)
            .fetch_all(self.as_ref())
            .await?;

        for row in rows {
            let id: String = row.get("id");
            let name: String = row.get("name");
            println!("id: {}, name: {}", id, name);
        }

        Ok(())
    }

    async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        MIGRATOR
            .run(self.as_ref())
            .await?;

        println!("run migrations for sqlite server");

        Ok(())
    }
}