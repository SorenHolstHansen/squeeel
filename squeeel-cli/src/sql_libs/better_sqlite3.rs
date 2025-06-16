use super::SupportedLib;
use crate::utils::ts_types::{
    TS_NULL_TYPE, TS_NUMBER_TYPE, TS_STRING_TYPE, TS_UNKNOWN_TYPE, ts_type_ref,
};
use crate::{sql_libs::SqlLib, visitor::Query};
use sqlx_core::type_info::TypeInfo;
use swc_common::BytePos;
use swc_ecma_ast::{ModuleItem, TsType};
use swc_ecma_parser::{Lexer, Parser, StringInput, Syntax, TsSyntax};

pub struct BetterSqlite3;

impl SqlLib for BetterSqlite3 {
    type Db = sqlx::Sqlite;

    fn parse_call_expr(&self, call_expr: &swc_ecma_ast::CallExpr) -> Option<crate::visitor::Query> {
        let swc_ecma_ast::Callee::Expr(expr) = &call_expr.callee else {
            return None;
        };

        let swc_ecma_ast::Expr::Member(member_expr) = &**expr else {
            return None;
        };

        let obj = &member_expr.obj.as_ident()?.sym;
        let prop = &member_expr.prop.as_ident()?.sym;
        if obj != "db" || prop != "prepare" {
            return None;
        }

        let mut args_iter = call_expr.args.iter();
        let query_expr = args_iter.next()?;
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
            lib: SupportedLib::BetterSqlite3,
        })
    }

    fn db_type_to_ts_type(&self, ty: &<Self::Db as sqlx::Database>::TypeInfo) -> TsType {
        match ty.name().to_lowercase().as_str() {
            "int" | "float" => TS_NUMBER_TYPE,
            "text" => TS_STRING_TYPE,
            "blob" => ts_type_ref("Buffer"),
            "null" => TS_NULL_TYPE,
            _ => TS_UNKNOWN_TYPE,
        }
    }

    fn d_ts_prefix(&self) -> Vec<ModuleItem> {
        vec![]
    }

    fn d_ts_suffix(&self) -> Vec<ModuleItem> {
        let suffix = r#"declare module 'better-sqlite3' {
    type VariableArgFunction = (...params: any[]) => unknown;
    type ArgumentTypes<F extends VariableArgFunction> = F extends (...args: infer A) => unknown ? A : never;
    type ElementOf<T> = T extends Array<infer E> ? E : T;

    interface Statement<BindParameters extends unknown[], Result = unknown> {
        database: Database;
        source: string;
        reader: boolean;
        readonly: boolean;
        busy: boolean;

        run(...params: BindParameters): Database.RunResult;
        get(...params: BindParameters): Result | undefined;
        all(...params: BindParameters): Result[];
        iterate(...params: BindParameters): IterableIterator<Result>;
        pluck(toggleState?: boolean): this;
        expand(toggleState?: boolean): this;
        raw(toggleState?: boolean): this;
        bind(...params: BindParameters): this;
        columns(): ColumnDefinition[];
        safeIntegers(toggleState?: boolean): this;
    }

    interface ColumnDefinition {
        name: string;
        column: string | null;
        table: string | null;
        database: string | null;
        type: string | null;
    }

    interface Transaction<F extends VariableArgFunction> {
        (...params: ArgumentTypes<F>): ReturnType<F>;
        default(...params: ArgumentTypes<F>): ReturnType<F>;
        deferred(...params: ArgumentTypes<F>): ReturnType<F>;
        immediate(...params: ArgumentTypes<F>): ReturnType<F>;
        exclusive(...params: ArgumentTypes<F>): ReturnType<F>;
    }

    interface VirtualTableOptions {
        rows: (...params: unknown[]) => Generator;
        columns: string[];
        parameters?: string[];
        safeIntegers?: boolean;
        directOnly?: boolean;
    }

    interface Database {
        memory: boolean;
        readonly: boolean;
        name: string;
        open: boolean;
        inTransaction: boolean;

        prepare<T extends string>(
            source: T
        ): Statement<T extends keyof Queries ? Queries[T]["args"] : unknown[], T extends keyof Queries ? Queries[T]["returnType"] : unknown>;
        transaction<F extends VariableArgFunction>(fn: F): Transaction<F>;
        exec(source: string): this;
        pragma(source: string, options?: Database.PragmaOptions): unknown;
        function(name: string, cb: (...params: any[]) => any): this;
        function(name: string, options: Database.RegistrationOptions, cb: (...params: any[]) => any): this;
        aggregate<T>(
            name: string,
            options: Database.RegistrationOptions & {
                start?: T | (() => T);
                step: (total: T, next: ElementOf<T>) => T | void;
                inverse?: ((total: T, dropped: T) => T) | undefined;
                result?: ((total: T) => unknown) | undefined;
            }
        ): this;
        loadExtension(path: string): this;
        close(): this;
        defaultSafeIntegers(toggleState?: boolean): this;
        backup(destinationFile: string, options?: Database.BackupOptions): Promise<Database.BackupMetadata>;
        table(name: string, options: VirtualTableOptions): this;
        unsafeMode(unsafe?: boolean): this;
        serialize(options?: Database.SerializeOptions): Buffer;
    }

    interface DatabaseConstructor {
        new (filename?: string | Buffer, options?: Database.Options): Database;
        (filename?: string, options?: Database.Options): Database;
        prototype: Database;
        SqliteError: SqliteErrorType;
    }

    class SqliteErrorClass extends Error {
        name: string;
        message: string;
        code: string;
        constructor(message: string, code: string);
    }

    type SqliteErrorType = typeof SqliteErrorClass;

    namespace Database {
        interface RunResult {
            changes: number;
            lastInsertRowid: number | bigint;
        }

        interface Options {
            readonly?: boolean;
            fileMustExist?: boolean;
            timeout?: number;
            verbose?: ((message?: unknown, ...additionalArgs: unknown[]) => void) | undefined;
            nativeBinding?: string;
        }

        interface SerializeOptions {
            attached?: string;
        }

        interface PragmaOptions {
            simple?: boolean;
        }

        interface RegistrationOptions {
            varargs?: boolean;
            deterministic?: boolean;
            safeIntegers?: boolean;
            directOnly?: boolean;
        }

        interface BackupMetadata {
            totalPages: number;
            remainingPages: number;
        }

        interface BackupOptions {
            progress: (info: BackupMetadata) => number;
        }
    }

    const Database: DatabaseConstructor;
    export = Database;
}"#;

        let lexer = Lexer::new(
            Syntax::Typescript(TsSyntax {
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
