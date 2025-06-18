use sqlx::{Executor, MySql, MySqlPool, Postgres, Sqlite};
use sqlx::{PgPool, SqlitePool};
use sqlx_core::describe::Describe;
use tokio::sync::OnceCell;

pub trait DbExt: sqlx::Database {
    type Db: sqlx::Database;

    async fn describe(query: String) -> Result<Describe<Self::Db>, sqlx::Error>;
}

static PG_POOL: OnceCell<PgPool> = OnceCell::const_new();
pub async fn init_pg_pool(database_url: &str) -> anyhow::Result<()> {
    PG_POOL.set(PgPool::connect(database_url).await?)?;
    Ok(())
}

pub fn pg_pool() -> &'static PgPool {
    PG_POOL.get().unwrap()
}

static SQLITE_POOL: OnceCell<SqlitePool> = OnceCell::const_new();
pub async fn init_sqlite_pool(database_url: &str) -> anyhow::Result<()> {
    SQLITE_POOL.set(SqlitePool::connect(database_url).await?)?;
    Ok(())
}

pub fn sqlite_pool() -> &'static SqlitePool {
    SQLITE_POOL.get().unwrap()
}

static MY_SQL_POOL: OnceCell<MySqlPool> = OnceCell::const_new();
pub async fn init_my_sql_pool(database_url: &str) -> anyhow::Result<()> {
    MY_SQL_POOL.set(MySqlPool::connect(database_url).await?)?;
    Ok(())
}

pub fn my_sql_pool() -> &'static MySqlPool {
    MY_SQL_POOL.get().unwrap()
}

impl DbExt for Postgres {
    type Db = Postgres;

    async fn describe(query: String) -> Result<Describe<Self::Db>, sqlx::Error> {
        pg_pool().describe(&query).await
    }
}

impl DbExt for Sqlite {
    type Db = Sqlite;

    async fn describe(query: String) -> Result<Describe<Self::Db>, sqlx::Error> {
        sqlite_pool().describe(&query).await
    }
}

impl DbExt for MySql {
    type Db = MySql;

    async fn describe(query: String) -> Result<Describe<Self::Db>, sqlx::Error> {
        my_sql_pool().describe(&query).await
    }
}
