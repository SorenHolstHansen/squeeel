mod node_postgres;
use crate::visitor::Query;
use sqlx::{Column, Either, Executor, MySql, Postgres, Sqlite};
use sqlx::{PgPool, TypeInfo};
use sqlx_core::describe::Describe;
use sqlx_core::type_info;
use swc_ecma_ast::CallExpr;
use tokio::sync::OnceCell;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SupportedLib {
    NodePostgres,
    BetterSqlite3,
}

impl std::fmt::Display for SupportedLib {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportedLib::NodePostgres => write!(f, "pg"),
            SupportedLib::BetterSqlite3 => write!(f, "better-sqlite3"),
        }
    }
}

impl TryFrom<String> for SupportedLib {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "pg" => Ok(SupportedLib::NodePostgres),
            _ => Err(()),
        }
    }
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

trait DbExt: sqlx::Database {
    type Db: sqlx::Database;

    async fn describe(query: String) -> Result<Describe<Self::Db>, sqlx::Error>;
}

impl DbExt for Postgres {
    type Db = Postgres;

    async fn describe(query: String) -> Result<Describe<Self::Db>, sqlx::Error> {
        pg_pool().await.describe(&query).await
    }
}

trait SqlLib {
    type Db: DbExt;

    fn parse_call_expr(&self, call_expr: &CallExpr) -> Option<Query>;

    fn d_ts_prefix(&self) -> &'static str;

    fn d_ts_suffix(&self) -> &'static str;

    fn db_type_to_ts_type(
        &self,
        ty: &<<Self::Db as DbExt>::Db as sqlx::Database>::TypeInfo,
    ) -> &'static str;
}

fn describe_to_d_ts_query<Lib: SqlLib>(
    lib: &Lib,
    describe: &Describe<<Lib::Db as DbExt>::Db>,
) -> (String, String) {
    let mut return_type = "{\n".to_string();
    for i in 0..describe.columns.len() {
        let column = &describe.columns[i];
        let nullable = &describe.nullable[i].unwrap_or(true);
        let type_info = column.type_info();
        let ts_type = lib.db_type_to_ts_type(type_info);
        let final_type = if *nullable {
            format!("{} | null", ts_type)
        } else {
            ts_type.to_string()
        };
        return_type.push_str(&format!(
            "            /** 
             * Native data type: `{}`.
             */\n",
            type_info.name(),
        ));
        return_type.push_str(&format!("            {}: {},\n", column.name(), final_type));
    }
    return_type.push_str("        }");

    let mut args = String::new();
    if let Some(params) = &describe.parameters {
        args.push('[');
        match params {
            Either::Left(params) => {
                for param in params.iter() {
                    let ts_type = lib.db_type_to_ts_type(param);

                    args.push_str(ts_type);
                }
            }
            Either::Right(count) => {
                todo!()
            }
        }
        args.push(']');
    } else {
        args.push_str("unknown");
    }

    (return_type, args)
}

async fn describe_bulk<Db: DbExt>(queries: Vec<String>) -> Vec<Describe<<Db as DbExt>::Db>> {
    let tasks = queries.into_iter().map(|query| Db::describe(query));

    let mut outputs = Vec::with_capacity(tasks.len());
    for task in tasks {
        outputs.push(task.await.unwrap());
    }

    outputs
}

async fn create_d_ts_file<Lib: SqlLib>(lib: Lib, queries: Vec<String>) -> String {
    let mut queries_type = "{\n".to_string();
    let descriptions = describe_bulk::<Lib::Db>(queries.clone()).await;
    for (i, describe) in descriptions.into_iter().enumerate() {
        let query = &queries[i];
        let (return_type, args) = describe_to_d_ts_query(&lib, &describe);
        queries_type.push_str(&format!(
            r#"    [`{}`]: {{
    returnType: {},
    args: {}
}},"#,
            query, return_type, args
        ));
        queries_type.push('\n');
    }

    queries_type.push_str("\n}");

    format!(
        "{}

type Queries = {};

{}",
        lib.d_ts_prefix(),
        queries_type,
        lib.d_ts_suffix()
    )
}

impl SupportedLib {
    pub fn parse_call_expr(&self, call_expr: &CallExpr) -> Option<Query> {
        match self {
            SupportedLib::NodePostgres => node_postgres::NodePostgres.parse_call_expr(call_expr),
            SupportedLib::BetterSqlite3 => todo!(),
        }
    }

    pub async fn create_d_ts_file(&self, queries: Vec<String>) -> String {
        match self {
            SupportedLib::NodePostgres => {
                create_d_ts_file(node_postgres::NodePostgres, queries).await
            }
            SupportedLib::BetterSqlite3 => todo!(),
        }
    }
}
