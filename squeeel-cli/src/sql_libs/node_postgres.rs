use super::SupportedLib;
use crate::{sql_libs::SqlLib, visitor::Query};
use sqlx::Column;
use sqlx_core::type_info::TypeInfo;
use sqlx_postgres::PgTypeInfo;

pub struct NodePostgres;

fn pg_type_to_ts_type(ty: &PgTypeInfo) -> &'static str {
    match ty.name().to_lowercase().as_str() {
        "bool" => "boolean",
        "line" | "polygon" | "path" | "lseg" | "jsonpath" | "tsrange" | "int4range"
        | "numrange" | "int8range" | "tstzrange" | "daterange" | "box" | "uuid" | "varbit"
        | "bit" | "numeric" | "text" | "varchar" | "bpchar" | "cidr" | "inet" | "int8" | "time"
        | "timetz" | "money" | "name" | "char" | "macaddr" | "macaddr8" => "string",
        "float4" | "float8" | "int2" | "int4" | "oid" => "number",
        "timestamp" | "timestamptz" | "Date" => "Date",
        "point" => "{x: number, y: number}",
        "jsonb" | "Json" => "JsonValue",
        "interval" => {
            "{
        milliseconds?: number;
        seconds?: number;
        minutes?: number;
        hours?: number;
        days?: number;
        months?: number;
        years?: number;
        toPostgres: Function;
        toISO: Function;
        toISOString: Function;
    }"
        }
        "bytea" => "Buffer",
        "circle" => {
            "{
        x: number;
        y: number;
        radius: number;
    }"
        }
        _ => "unknown",
    }
}

impl SqlLib for NodePostgres {
    type Db = sqlx::Postgres;

    fn parse_call_expr(&self, call_expr: &swc_ecma_ast::CallExpr) -> Option<crate::visitor::Query> {
        let swc_ecma_ast::Callee::Expr(expr) = &call_expr.callee else {
            return None;
        };

        let swc_ecma_ast::Expr::Member(member_expr) = &**expr else {
            return None;
        };

        let obj = &member_expr.obj.as_ident()?.sym;
        let prop = &member_expr.prop.as_ident()?.sym;
        if obj != "client" || prop != "query" {
            return None;
        }

        let mut args_iter = call_expr.args.iter();
        let query_expr = args_iter.next()?;
        let _args = args_iter.next();
        if args_iter.next().is_some() {
            return None;
        }

        if query_expr.spread.is_some() {
            return None;
        }

        let query = &query_expr.expr.as_lit()?.as_str()?.value;

        Some(Query {
            query: query.to_string(),
            lib: SupportedLib::NodePostgres,
        })
    }

    fn generate_declaration_file_content(
        &self,
        statements: Vec<(sqlx::Describe<Self::Db>, Query)>,
    ) -> String {
        let mut queries_type = "{\n".to_string();
        for (describe, query) in statements {
            let mut return_type = "{".to_string();
            let args = "[]";
            for i in 0..describe.columns.len() {
                let column = &describe.columns[i];
                let nullable = describe.nullable[i].unwrap_or(false);
                let mut ty = pg_type_to_ts_type(column.type_info()).to_string();
                if nullable {
                    ty = format!("{} | undefined", ty);
                }
                let column_name = column.name();
                return_type.push_str(&format!(r#""{}": {},"#, column_name, ty));
            }
            return_type.push('}');

            queries_type.push_str(&format!(
                r#""{}": {{ "args": {}, "returnType": {} }},"#,
                query.query, args, return_type
            ));
        }

        queries_type.push('}');

        format!(
            r#"import type {{ QueryResult }} from "pg";

type JsonValue = string | number | boolean | null | {{ [Key in string]?: JsonValue }} | JsonValue[];

export type Queries = {queries_type};

declare module 'pg' {{
	export interface ClientBase {{
		query<T extends string>(
            q: T, 
            args: T extends keyof Queries ? Queries[T]["args"] : unknown
        ): Promise<T extends keyof Queries ? QueryResult<Queries[T]["returnType"]> : unknown>;
    }}
}}"#
        )
    }
}
