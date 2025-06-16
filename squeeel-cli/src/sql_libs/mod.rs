mod node_postgres;
use crate::utils::ts_types::{
    TS_UNKNOWN_TYPE, ts_object_type, ts_object_type_computed, ts_optional_type, ts_tuple_type,
};
use crate::visitor::Query;
use sqlx::PgPool;
use sqlx::{Column, Either, Executor, MySql, Postgres, Sqlite};
use sqlx_core::describe::Describe;
use swc_common::Span;
use swc_ecma_ast::{
    CallExpr, Decl, Expr, Ident, Module, ModuleItem, Stmt, Tpl, TplElement, TsType, TsTypeAliasDecl,
};
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

    fn d_ts_prefix(&self) -> Vec<ModuleItem>;

    fn d_ts_suffix(&self) -> Vec<ModuleItem>;

    fn db_type_to_ts_type(
        &self,
        ty: &<<Self::Db as DbExt>::Db as sqlx::Database>::TypeInfo,
    ) -> TsType;
}

fn describe_to_d_ts_query<Lib: SqlLib>(
    lib: &Lib,
    describe: &Describe<<Lib::Db as DbExt>::Db>,
) -> (TsType, TsType) {
    let mut return_type_members: Vec<(Expr, TsType, bool)> =
        Vec::with_capacity(describe.columns.len());
    for i in 0..describe.columns.len() {
        let column = &describe.columns[i];
        let nullable = &describe.nullable[i].unwrap_or(true);
        let type_info = column.type_info();
        let ts_type = lib.db_type_to_ts_type(type_info);
        let final_type = if *nullable {
            ts_optional_type(ts_type)
        } else {
            ts_type
        };
        return_type_members.push((column.name().into(), final_type, *nullable));
    }

    let mut args = Vec::new();
    if let Some(params) = &describe.parameters {
        match params {
            Either::Left(params) => {
                for param in params.iter() {
                    let ts_type = lib.db_type_to_ts_type(param);

                    args.push(ts_type);
                }
            }
            Either::Right(count) => {
                for _ in 0..*count {
                    // For a fixed number of parameters, we assume they are all of type `unknown`
                    args.push(TS_UNKNOWN_TYPE);
                }
            }
        }
    } else {
        // If there are no parameters, we assume an empty tuple
    }

    (ts_object_type(return_type_members), ts_tuple_type(args))
}

async fn describe_bulk<Db: DbExt>(queries: Vec<String>) -> Vec<Describe<<Db as DbExt>::Db>> {
    let tasks = queries.into_iter().map(|query| Db::describe(query));

    let mut outputs = Vec::with_capacity(tasks.len());
    for task in tasks {
        outputs.push(task.await.unwrap());
    }

    outputs
}

async fn create_d_ts_file<Lib: SqlLib>(lib: Lib, queries: Vec<String>) -> Module {
    let descriptions = describe_bulk::<Lib::Db>(queries.clone()).await;
    let mut queries_type_members: Vec<(Expr, TsType, bool)> = Vec::with_capacity(queries.len());
    for (i, describe) in descriptions.into_iter().enumerate() {
        let query = &queries[i];
        let (return_type, args) = describe_to_d_ts_query(&lib, &describe);
        queries_type_members.push((
            Expr::Tpl(Tpl {
                span: Span::default(),
                exprs: Vec::new(),
                quasis: vec![TplElement {
                    span: Span::default(),
                    tail: true,
                    cooked: None,
                    raw: query.to_string().into(),
                }],
            }),
            ts_object_type([
                ("returnType".into(), return_type, false),
                ("args".into(), args, false),
            ]),
            false,
        ));
    }

    let mut body = Vec::new();
    body.extend(lib.d_ts_prefix());
    body.push(ModuleItem::Stmt(Stmt::Decl(Decl::TsTypeAlias(Box::new(
        TsTypeAliasDecl {
            span: Span::default(),
            declare: false,
            id: Ident::new_no_ctxt("Queries".into(), Span::default()),
            type_params: None,
            type_ann: Box::new(ts_object_type_computed(queries_type_members)),
        },
    )))));
    body.extend(lib.d_ts_suffix());
    Module {
        span: Span::default(),
        body,
        shebang: None,
    }
}

impl SupportedLib {
    pub fn parse_call_expr(&self, call_expr: &CallExpr) -> Option<Query> {
        match self {
            SupportedLib::NodePostgres => node_postgres::NodePostgres.parse_call_expr(call_expr),
            SupportedLib::BetterSqlite3 => todo!(),
        }
    }

    pub async fn create_d_ts_file(&self, queries: Vec<String>) -> Module {
        match self {
            SupportedLib::NodePostgres => {
                create_d_ts_file(node_postgres::NodePostgres, queries).await
            }
            SupportedLib::BetterSqlite3 => todo!(),
        }
    }
}
