import type mysql from "mysql2/promise";
type JsonValue = string | number | boolean | null | {
    [Key in string]?: JsonValue;
} | JsonValue[];
type Tables = {
    "a": {
        "id": number;
        "char_col"?: string | undefined;
        "varchar_col"?: string | undefined;
        "binary_col"?: Buffer | undefined;
        "varbinary_col"?: Buffer | undefined;
        "text_col"?: string | undefined;
        "blob_col"?: Buffer | undefined;
        "tinyint_col"?: number | undefined;
        "smallint_col"?: number | undefined;
        "mediumint_col"?: number | undefined;
        "int_col"?: number | undefined;
        "bigint_col"?: number | undefined;
        "float_col"?: number | undefined;
        "double_col"?: number | undefined;
        "decimal_col"?: string | undefined;
        "date_col"?: Date | undefined;
        "time_col"?: string | undefined;
        "datetime_col"?: Date | undefined;
        "timestamp_col"?: Date | undefined;
        "bool_col"?: number | undefined;
        "json_col"?: JsonValue | undefined;
    };
};
type Queries = {
    [`
		SELECT
			char_col,
			varchar_col,
			bool_col,
			tinyint_col,
			smallint_col,
			mediumint_col,
			int_col,
			bigint_col,
			float_col,
			double_col,
			decimal_col,
			NULL AS null_value,
			binary_col,
			varbinary_col,
			date_col,
			time_col,
			datetime_col,
			timestamp_col,
			text_col,
			blob_col,
			json_col,
			UUID() AS uuid_col,
			INET_ATON('192.168.1.1') AS inet4_as_int_col, -- No native inet type, this gives INT representation
			CAST(123456 AS UNSIGNED) AS unsigned_int_col
		FROM a;
			`]: {
        "returnType": {
            "char_col"?: string | undefined;
            "varchar_col"?: string | undefined;
            "bool_col"?: number | undefined;
            "tinyint_col"?: number | undefined;
            "smallint_col"?: number | undefined;
            "mediumint_col"?: number | undefined;
            "int_col"?: number | undefined;
            "bigint_col"?: number | undefined;
            "float_col"?: number | undefined;
            "double_col"?: number | undefined;
            "decimal_col"?: string | undefined;
            "null_value"?: null | undefined;
            "binary_col"?: Buffer | undefined;
            "varbinary_col"?: Buffer | undefined;
            "date_col"?: Date | undefined;
            "time_col"?: string | undefined;
            "datetime_col"?: Date | undefined;
            "timestamp_col"?: Date | undefined;
            "text_col"?: string | undefined;
            "blob_col"?: Buffer | undefined;
            "json_col"?: JsonValue | undefined;
            "uuid_col"?: string | undefined;
            "inet4_as_int_col"?: number | undefined;
            "unsigned_int_col": number;
        };
        "args": never;
    };
    [`SELECT ? as input`]: {
        "returnType": {
            "input"?: string | undefined;
        };
        "args": [unknown];
    };
};
declare module "mysql2/promise" {
    export interface Connection {
        query<T extends string>(...params: T extends keyof Queries ? Queries[T]["args"] extends never ? [sql: T] : [sql: T, values: Queries[T]["args"]] : [sql: T, values: any]): Promise<[T extends keyof Queries ? Queries[T]["returnType"][] : unknown, mysql.FieldPacket[]]>;
    }
}
