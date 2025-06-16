use super::SupportedLib;
use crate::utils::ts_types::{
    TS_BOOLEAN_TYPE, TS_NULL_TYPE, TS_NUMBER_TYPE, TS_STRING_TYPE, TS_UNKNOWN_TYPE, ts_object_type,
    ts_type_ref,
};
use crate::{sql_libs::SqlLib, visitor::Query};
use sqlx_core::type_info::TypeInfo;
use swc_common::BytePos;
use swc_ecma_ast::{ModuleItem, TsType};
use swc_ecma_parser::{Lexer, Parser, StringInput, Syntax, TsSyntax};

pub struct MySql2;

impl SqlLib for MySql2 {
    type Db = sqlx::MySql;

    fn parse_call_expr(&self, call_expr: &swc_ecma_ast::CallExpr) -> Option<String> {
        let swc_ecma_ast::Callee::Expr(expr) = &call_expr.callee else {
            return None;
        };

        let swc_ecma_ast::Expr::Member(member_expr) = &**expr else {
            return None;
        };

        let obj = &member_expr.obj.as_ident()?.sym;
        let prop = &member_expr.prop.as_ident()?.sym;
        if obj != "connection" || prop != "query" {
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
            "text" | "char" | "varchar" | "decimal" | "time" => TS_STRING_TYPE,
            "boolean" | "tinyint" | "smallint" | "mediumint" | "int" | "bigint" | "float"
            | "double" | "bigint unsigned" => TS_NUMBER_TYPE,
            "null" => TS_NULL_TYPE,
            "binary" | "varbinary" | "blob" => ts_type_ref("Buffer"),
            "date" | "datetime" | "timestamp" => ts_type_ref("Date"),
            "json" => ts_type_ref("JsonValue"),
            _ => TS_UNKNOWN_TYPE,
        }
    }

    fn d_ts_prefix(&self) -> Vec<ModuleItem> {
        let prefix = r#"import type mysql from "mysql2/promise";

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
        let suffix = r#"declare module "mysql2/promise" {
    export interface Connection {
        query<T extends string>(
			...params: T extends keyof Queries ? 
				Queries[T]["args"] extends never ? 
					[sql: T] : 
					[sql: T, values: Queries[T]["args"]] : 
				[sql: T, values: any]
		): Promise<[T extends keyof Queries ? Queries[T]["returnType"][] : unknown, mysql.FieldPacket[]]>;
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
