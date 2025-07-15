mod better_sqlite3;
mod mysql2;
mod node_postgres;
use crate::describe::DbExt;
use crate::utils::ts_types::{
    TS_NEVER_TYPE, TS_UNKNOWN_TYPE, ts_object_type, ts_object_type_computed, ts_optional_type,
    ts_tuple_type,
};
use crate::visitor::Query;
use sqlx::{Column, Either};
use sqlx_core::describe::Describe;
use swc_common::Span;
use swc_ecma_ast::{
    CallExpr, Decl, Expr, Ident, Module, ModuleItem, Stmt, Tpl, TplElement, TsType, TsTypeAliasDecl,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SupportedLib {
    NodePostgres,
    BetterSqlite3,
    MySql2,
}

#[derive(PartialEq, Eq, Hash)]
pub enum Dialect {
    Postgres,
    MySql,
    Sqlite,
}

impl SupportedLib {
    pub fn dialect(&self) -> Dialect {
        match self {
            SupportedLib::NodePostgres => Dialect::Postgres,
            SupportedLib::BetterSqlite3 => Dialect::Sqlite,
            SupportedLib::MySql2 => Dialect::MySql,
        }
    }
}

impl std::fmt::Display for SupportedLib {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportedLib::NodePostgres => write!(f, "pg"),
            SupportedLib::BetterSqlite3 => write!(f, "better-sqlite3"),
            SupportedLib::MySql2 => write!(f, "mysql2"),
        }
    }
}

impl TryFrom<String> for SupportedLib {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "pg" => Ok(SupportedLib::NodePostgres),
            "better-sqlite3" => Ok(SupportedLib::BetterSqlite3),
            "mysql2" => Ok(SupportedLib::MySql2),
            _ => Err(()),
        }
    }
}

trait SqlLib {
    type Db: DbExt;

    fn parse_call_expr(&self, call_expr: &CallExpr) -> Option<String>;

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
                // For a fixed number of parameters, we assume they are all of type `unknown`
                args = vec![TS_UNKNOWN_TYPE; *count];
            }
        }
    } else {
        // If there are no parameters, we assume an empty tuple
    }

    (
        ts_object_type(return_type_members),
        if args.is_empty() {
            TS_NEVER_TYPE
        } else {
            ts_tuple_type(args)
        },
    )
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

    let table_names = Lib::Db::get_table_names().await.unwrap();
    let descriptions = describe_bulk::<Lib::Db>(
        table_names
            .iter()
            .map(|table_name| format!("SELECT * FROM {table_name}"))
            .collect::<Vec<_>>(),
    )
    .await;
    let mut table_types: Vec<(Expr, TsType, bool)> = Vec::with_capacity(table_names.len());
    for (i, describe) in descriptions.into_iter().enumerate() {
        let table_name = table_names[i].clone();

        let (return_type, _) = describe_to_d_ts_query(&lib, &describe);
        table_types.push((table_name.into(), return_type, false));
    }

    let mut body = Vec::new();
    body.extend(lib.d_ts_prefix());
    body.push(ModuleItem::Stmt(Stmt::Decl(Decl::TsTypeAlias(Box::new(
        TsTypeAliasDecl {
            span: Span::default(),
            declare: false,
            id: Ident::new_no_ctxt("Tables".into(), Span::default()),
            type_params: None,
            type_ann: Box::new(ts_object_type(table_types)),
        },
    )))));
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
        let query = match self {
            SupportedLib::NodePostgres => node_postgres::NodePostgres.parse_call_expr(call_expr),
            SupportedLib::BetterSqlite3 => better_sqlite3::BetterSqlite3.parse_call_expr(call_expr),
            SupportedLib::MySql2 => mysql2::MySql2.parse_call_expr(call_expr),
        };
        query.map(|q| Query {
            query: q,
            lib: *self,
        })
    }

    pub async fn create_d_ts_file(&self, queries: Vec<String>) -> Module {
        match self {
            SupportedLib::NodePostgres => {
                create_d_ts_file(node_postgres::NodePostgres, queries).await
            }
            SupportedLib::BetterSqlite3 => {
                create_d_ts_file(better_sqlite3::BetterSqlite3, queries).await
            }
            SupportedLib::MySql2 => create_d_ts_file(mysql2::MySql2, queries).await,
        }
    }
}
