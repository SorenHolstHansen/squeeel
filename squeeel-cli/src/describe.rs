use sqlx::{Executor, MySql, MySqlPool, Postgres, Sqlite};
use sqlx::{PgPool, SqlitePool};
use sqlx_core::describe::Describe;
use tokio::sync::OnceCell;

pub trait DbExt: sqlx::Database {
    type Db: sqlx::Database;

    async fn describe(query: String) -> Result<Describe<Self::Db>, sqlx::Error>;
}

static PG_POOL: OnceCell<PgPool> = OnceCell::const_new();
pub async fn pg_pool() -> &'static PgPool {
    PG_POOL
        .get_or_init(|| async {
            // TODO: Get this from env vars, or a config file, or a command line argument, or something
            PgPool::connect("postgres://postgres:postgres@localhost:5432/postgres")
                .await
                .unwrap()
        })
        .await
}

static SQLITE_POOL: OnceCell<SqlitePool> = OnceCell::const_new();
pub async fn sqlite_pool() -> &'static SqlitePool {
    SQLITE_POOL
        .get_or_init(|| async {
            // TODO: Get this from env vars, or a config file, or a command line argument, or something
            SqlitePool::connect("../examples/better-sqlite3/example.db")
                .await
                .unwrap()
        })
        .await
}

static MY_SQL_POOL: OnceCell<MySqlPool> = OnceCell::const_new();
pub async fn my_sql_pool() -> &'static MySqlPool {
    MY_SQL_POOL
        .get_or_init(|| async {
            // TODO: Get this from env vars, or a config file, or a command line argument, or something
            MySqlPool::connect("my-database.db").await.unwrap()
        })
        .await
}

impl DbExt for Postgres {
    type Db = Postgres;

    async fn describe(query: String) -> Result<Describe<Self::Db>, sqlx::Error> {
        pg_pool().await.describe(&query).await
    }
}

impl DbExt for Sqlite {
    type Db = Sqlite;

    async fn describe(query: String) -> Result<Describe<Self::Db>, sqlx::Error> {
        sqlite_pool().await.describe(&query).await
    }
}

impl DbExt for MySql {
    type Db = MySql;

    async fn describe(query: String) -> Result<Describe<Self::Db>, sqlx::Error> {
        my_sql_pool().await.describe(&query).await
    }
}
