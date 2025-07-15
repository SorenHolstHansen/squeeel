use sqlx::{Executor, MySql, MySqlPool, Postgres, Sqlite};
use sqlx::{PgPool, SqlitePool};
use sqlx_core::describe::Describe;
use tokio::sync::OnceCell;

pub trait DbExt: sqlx::Database {
    type Db: sqlx::Database;

    async fn describe(query: String) -> Result<Describe<Self::Db>, sqlx::Error>;
    async fn get_table_names() -> Result<Vec<String>, sqlx::Error>;
}

static PG_POOL: OnceCell<PgPool> = OnceCell::const_new();
pub async fn init_pg_pool(database_url: &str) -> anyhow::Result<()> {
    PG_POOL.set(PgPool::connect(database_url).await?)?;
    Ok(())
}

static SQLITE_POOL: OnceCell<SqlitePool> = OnceCell::const_new();
pub async fn init_sqlite_pool(database_url: &str) -> anyhow::Result<()> {
    SQLITE_POOL.set(SqlitePool::connect(database_url).await?)?;
    Ok(())
}

static MY_SQL_POOL: OnceCell<MySqlPool> = OnceCell::const_new();
pub async fn init_my_sql_pool(database_url: &str) -> anyhow::Result<()> {
    MY_SQL_POOL.set(MySqlPool::connect(database_url).await?)?;
    Ok(())
}

impl DbExt for Postgres {
    type Db = Postgres;

    async fn describe(query: String) -> Result<Describe<Self::Db>, sqlx::Error> {
        PG_POOL.get().unwrap().describe(&query).await
    }

    async fn get_table_names() -> Result<Vec<String>, sqlx::Error> {
        let pool = PG_POOL.get().unwrap();
        let table_names: Vec<String> = sqlx::query_scalar("SELECT tablename FROM pg_catalog.pg_tables WHERE schemaname != 'pg_catalog' AND schemaname != 'information_schema';").fetch_all(pool).await?;

        Ok(table_names)
    }
}

impl DbExt for Sqlite {
    type Db = Sqlite;

    async fn describe(query: String) -> Result<Describe<Self::Db>, sqlx::Error> {
        SQLITE_POOL.get().unwrap().describe(&query).await
    }

    async fn get_table_names() -> Result<Vec<String>, sqlx::Error> {
        let pool = SQLITE_POOL.get().unwrap();
        let table_names: Vec<String> = sqlx::query_scalar(
            "SELECT name FROM sqlte_schema WHERE type = 'table' AND name NOT LIKE 'sqlite_%';",
        )
        .fetch_all(pool)
        .await?;

        Ok(table_names)
    }
}

impl DbExt for MySql {
    type Db = MySql;

    async fn describe(query: String) -> Result<Describe<Self::Db>, sqlx::Error> {
        MY_SQL_POOL.get().unwrap().describe(&query).await
    }

    async fn get_table_names() -> Result<Vec<String>, sqlx::Error> {
        let pool = MY_SQL_POOL.get().unwrap();
        let table_names: Vec<String> =
            sqlx::query_scalar("SELECT table_name FROM information_schema.tables;")
                .fetch_all(pool)
                .await?;

        Ok(table_names)
    }
}
