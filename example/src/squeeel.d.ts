import type { QueryResult } from 'pg';
type JsonValue = string | number | boolean | null | {
    [Key in string]?: JsonValue;
} | JsonValue[];
type Queries = {
    [`SELECT $1::text as message`]: {
        "returnType": {
            "message"?: string | undefined;
        };
        "args": [string];
    };
    [`
        SELECT 
		$1::text as message,
		NULL as null,
		true as bool,
		1::smallint as smallint,
		1::integer as integer,
		1::bigint as bigint,
		1.5::real as real,
		1.5::double precision as double,
		'c'::char as char,
		'hello' as string,
    	x'0010' as bit,
    	x'0010'::varbit as varbit,
		'\xDEADBEEF'::bytea as bytea,
		'(2,2),(0,0)'::box as box,
		POINT(1.2, 123.1) as point,
		'[(0,0),(1,1)]'::path as path,
		'((0,0),(1,1))'::polygon as polygon,
		LINE(POINT(1.2, 123.1), POINT(123.1, 1.2)) as line,
		LSEG(POINT(1.2, 123.1), POINT(123.1, 1.2)) as lseg,
		CIRCLE(POINT(1.2, 123.1), 10) as circle,
		'1 year 2 months 2 days 3 hours 20 minutes 1 second 20 milliseconds'::interval as interval,
		'{"hello": "world"}'::json as json,
		gen_random_uuid() as uuid,
		'2001-09-28'::date as date,
		'10.1.0.0/16'::cidr as cidr,
		'10.1.0.0/16'::inet as inet,
		'08:00:2b:01:02:03'::macaddr as macaddr,
		'08:00:2b:01:02:03:04:05'::macaddr8 as macaddr8,
		'12.34'::float8::numeric::money as numeric,
		'matt'::name as name,
		1::oid
        `]: {
        "returnType": {
            "message"?: string | undefined;
            "null"?: string | undefined;
            "bool"?: boolean | undefined;
            "smallint"?: number | undefined;
            "integer"?: number | undefined;
            "bigint"?: string | undefined;
            "real"?: number | undefined;
            "double"?: number | undefined;
            "char"?: string | undefined;
            "string"?: string | undefined;
            "bit"?: string | undefined;
            "varbit"?: string | undefined;
            "bytea"?: Buffer | undefined;
            "box"?: string | undefined;
            "point"?: {
                "x": number;
                "y": number;
            } | undefined;
            "path"?: string | undefined;
            "polygon"?: string | undefined;
            "line"?: string | undefined;
            "lseg"?: string | undefined;
            "circle"?: {
                "x": number;
                "y": number;
                "radius": number;
            } | undefined;
            "interval"?: {
                "milliseconds"?: number;
                "seconds"?: number;
                "minutes"?: number;
                "hours"?: number;
                "days"?: number;
                "months"?: number;
                "years"?: number;
            } | undefined;
            "json"?: JsonValue | undefined;
            "uuid"?: string | undefined;
            "date"?: Date | undefined;
            "cidr"?: string | undefined;
            "inet"?: string | undefined;
            "macaddr"?: string | undefined;
            "macaddr8"?: string | undefined;
            "numeric"?: string | undefined;
            "name"?: string | undefined;
            "oid"?: number | undefined;
        };
        "args": [string];
    };
};
declare module "pg" {
    export interface ClientBase {
        "query"<T extends string>(q: T, args: T extends keyof Queries ? Queries[T]["args"] : unknown): Promise<T extends keyof Queries ? QueryResult<Queries[T]["returnType"]> : unknown>;
    }
}
