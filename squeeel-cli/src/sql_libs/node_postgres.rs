use crate::sql_libs::SqlLib;
use crate::utils::ts_types::{
    TS_BOOLEAN_TYPE, TS_NUMBER_TYPE, TS_STRING_TYPE, TS_UNKNOWN_TYPE, ts_object_type, ts_type_ref,
};
use sqlx_core::type_info::TypeInfo;
use swc_common::BytePos;
use swc_ecma_ast::{ModuleItem, TsType};
use swc_ecma_parser::{Lexer, Parser, StringInput, Syntax, TsSyntax};

pub struct NodePostgres;

impl SqlLib for NodePostgres {
    type Db = sqlx::Postgres;

    fn parse_call_expr(&self, call_expr: &swc_ecma_ast::CallExpr) -> Option<String> {
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

        Some(query)
    }

    fn db_type_to_ts_type(&self, ty: &<Self::Db as sqlx::Database>::TypeInfo) -> TsType {
        match ty.name().to_lowercase().as_str() {
            "bool" => TS_BOOLEAN_TYPE,
            "line" | "polygon" | "path" | "lseg" | "jsonpath" | "tsrange" | "int4range"
            | "numrange" | "int8range" | "tstzrange" | "daterange" | "box" | "uuid" | "varbit"
            | "bit" | "numeric" | "text" | "varchar" | "bpchar" | "cidr" | "inet" | "int8"
            | "time" | "timetz" | "money" | "name" | "char" | "macaddr" | "macaddr8" => {
                TS_STRING_TYPE
            }
            "float4" | "float8" | "int2" | "int4" | "oid" => TS_NUMBER_TYPE,
            "timestamp" | "timestamptz" | "date" => ts_type_ref("Date"),
            "point" => ts_object_type([
                ("x".into(), TS_NUMBER_TYPE, false),
                ("y".into(), TS_NUMBER_TYPE, false),
            ]),
            "jsonb" | "json" => ts_type_ref("JsonValue"),
            "interval" => ts_object_type([
                ("milliseconds".into(), TS_NUMBER_TYPE, true),
                ("seconds".into(), TS_NUMBER_TYPE, true),
                ("minutes".into(), TS_NUMBER_TYPE, true),
                ("hours".into(), TS_NUMBER_TYPE, true),
                ("days".into(), TS_NUMBER_TYPE, true),
                ("months".into(), TS_NUMBER_TYPE, true),
                ("years".into(), TS_NUMBER_TYPE, true),
            ]),
            "bytea" => ts_type_ref("Buffer"),
            "circle" => ts_object_type([
                ("x".into(), TS_NUMBER_TYPE, false),
                ("y".into(), TS_NUMBER_TYPE, false),
                ("radius".into(), TS_NUMBER_TYPE, false),
            ]),
            _ => TS_UNKNOWN_TYPE,
        }
    }

    fn d_ts_prefix(&self) -> Vec<ModuleItem> {
        let prefix = r#"import type pg from "pg";
type JsonValue = string | number | boolean | null | {
    [Key in string]?: JsonValue;
} | JsonValue[];
"#;
        let lexer = Lexer::new(
            Syntax::Typescript(TsSyntax {
                ..Default::default()
            }),
            Default::default(),
            StringInput::new(prefix, BytePos(0), BytePos(prefix.len() as u32)),
            None,
        );
        let mut parser = Parser::new_from(lexer);
        parser.parse_typescript_module().unwrap().body
    }

    fn d_ts_suffix(&self) -> Vec<ModuleItem> {
        let suffix = r#"declare module "pg" {
    export interface ClientBase {
        query<T extends string>(
            ...params: T extends keyof Queries ? 
                Queries[T]["args"] extends never ? 
                    [q: T, callback: (err: Error, result: QueryResult<T extends keyof Queries ? Queries[T]["returnType"] : unknown>) => void,] : 
                    [q: T, args: Queries[T]["args"], callback: (err: Error, result: QueryResult<T extends keyof Queries ? Queries[T]["returnType"] : unknown>) => void,] 
                : [q: T, args: any, callback: (err: Error, result: QueryResult<T extends keyof Queries ? Queries[T]["returnType"] : unknown>) => void,]
        ): void;
        query<T extends string>(
            ...params: T extends keyof Queries ? 
                Queries[T]["args"] extends never ? 
                    [q: T] : 
                    [q: T, args: Queries[T]["args"]] 
                : [q: T, args: any]
        ): Promise<QueryResult<T extends keyof Queries ? Queries[T]["returnType"] : unknown>>;
    }
}
"#;
        let lexer = Lexer::new(
            Syntax::Typescript(TsSyntax {
                no_early_errors: true,
                ..Default::default()
            }),
            Default::default(),
            StringInput::new(suffix, BytePos(0), BytePos(suffix.len() as u32)),
            None,
        );
        let mut parser = Parser::new_from(lexer);
        parser.parse_typescript_module().unwrap().body
    }
}
