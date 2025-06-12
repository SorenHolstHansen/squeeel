mod node_postgres;
use crate::visitor::Query;
use node_postgres::NodePostgres;
use sqlx::Describe;
use swc_ecma_ast::CallExpr;

#[derive(PartialEq, Eq, PartialOrd, Hash)]
pub enum Dialect {
    Postgres,
    Sqlite,
    MySql,
}

impl From<&SupportedLib> for Dialect {
    fn from(value: &SupportedLib) -> Self {
        match value {
            SupportedLib::NodePostgres => Dialect::Postgres,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SupportedLib {
    NodePostgres,
}

impl std::fmt::Display for SupportedLib {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportedLib::NodePostgres => write!(f, "pg"),
        }
    }
}

impl TryFrom<String> for SupportedLib {
    type Error = bool;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "pg" => Ok(SupportedLib::NodePostgres),
            _ => Err(false),
        }
    }
}

trait SqlLib {
    type Db: sqlx::Database;

    fn parse_call_expr(&self, call_expr: &CallExpr) -> Option<Query>;
    fn generate_declaration_file_content(
        &self,
        statements: Vec<(Describe<Self::Db>, Query)>,
    ) -> String;
}

impl SupportedLib {
    pub fn parse_call_expr(&self, call_expr: &CallExpr) -> Option<Query> {
        match self {
            SupportedLib::NodePostgres => NodePostgres.parse_call_expr(call_expr),
        }
    }
}
