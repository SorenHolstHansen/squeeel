use super::SupportedLib;
use crate::{sql_libs::SqlLib, visitor::Query};
use sqlx_core::type_info::TypeInfo;

pub struct NodePostgres;

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

        let query = match &*query_expr.expr {
            swc_ecma_ast::Expr::Lit(lit) => lit.as_str()?.value.to_string(),
            swc_ecma_ast::Expr::Tpl(tpl) => tpl
                .quasis
                .iter()
                .map(|quasi| quasi.raw.to_string())
                .collect::<Vec<_>>()
                .join(""),
            _ => return None,
        };

        Some(Query {
            query: query.to_string(),
            lib: SupportedLib::NodePostgres,
        })
    }

    fn db_type_to_ts_type(&self, ty: &<Self::Db as sqlx::Database>::TypeInfo) -> &'static str {
        match ty.name().to_lowercase().as_str() {
            "bool" => "boolean",
            "line" | "polygon" | "path" | "lseg" | "jsonpath" | "tsrange" | "int4range"
            | "numrange" | "int8range" | "tstzrange" | "daterange" | "box" | "uuid" | "varbit"
            | "bit" | "numeric" | "text" | "varchar" | "bpchar" | "cidr" | "inet" | "int8"
            | "time" | "timetz" | "money" | "name" | "char" | "macaddr" | "macaddr8" => "string",
            "float4" | "float8" | "int2" | "int4" | "oid" => "number",
            "timestamp" | "timestamptz" | "date" => "Date",
            "point" => "{x: number, y: number}",
            "jsonb" | "json" => "JsonValue",
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

    fn d_ts_prefix(&self) -> &'static str {
        r#"import type { QueryResult } from "pg";

type JsonValue = string | number | boolean | null | { [Key in string]?: JsonValue } | JsonValue[];"#
    }

    fn d_ts_suffix(&self) -> &'static str {
        r#"declare module 'pg' {
    export interface ClientBase {
    	query<T extends string>(
            q: T,
            args: T extends keyof Queries ? Queries[T]["args"] : unknown
        ): Promise<T extends keyof Queries ? QueryResult<Queries[T]["returnType"]> : unknown>;
    }
}"#
    }
}
