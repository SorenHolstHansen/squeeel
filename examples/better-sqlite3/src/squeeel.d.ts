type Queries = {
    [`
        SELECT 
			NULL AS n,
			1 AS int,
			1.5 AS float,
			'hello' AS string,
			x'0010' AS buffer,
			? AS f
        `]: {
        "returnType": {
            "n"?: null | undefined;
            "int": unknown;
            "float": unknown;
            "string": string;
            "buffer": Buffer;
            "f"?: null | undefined;
        };
        "args": [unknown];
    };
};
declare module 'better-sqlite3' {
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
        (...params: ArgumentTypes<F>) : ReturnType<F>;
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
        prepare<T extends string>(source: T): Statement<T extends keyof Queries ? Queries[T]["args"] : unknown[], T extends keyof Queries ? Queries[T]["returnType"] : unknown>;
        transaction<F extends VariableArgFunction>(fn: F): Transaction<F>;
        exec(source: string): this;
        pragma(source: string, options?: Database.PragmaOptions): unknown;
        function(name: string, cb: (...params: any[]) => any): this;
        function(name: string, options: Database.RegistrationOptions, cb: (...params: any[]) => any): this;
        aggregate<T>(name: string, options: Database.RegistrationOptions & {
            start?: T | (() => T);
            step: (total: T, next: ElementOf<T>) => T | void;
            inverse?: ((total: T, dropped: T) => T) | undefined;
            result?: ((total: T) => unknown) | undefined;
        }): this;
        loadExtension(path: string): this;
        close(): this;
        defaultSafeIntegers(toggleState?: boolean): this;
        backup(destinationFile: string, options?: Database.BackupOptions): Promise<Database.BackupMetadata>;
        table(name: string, options: VirtualTableOptions): this;
        unsafeMode(unsafe?: boolean): this;
        serialize(options?: Database.SerializeOptions): Buffer;
    }
    interface DatabaseConstructor {
        new(filename?: string | Buffer, options?: Database.Options): Database;
        (filename?: string, options?: Database.Options) : Database;
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
    export = Database
}
