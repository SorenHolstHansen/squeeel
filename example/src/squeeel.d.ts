import type { QueryResult } from "pg";

type JsonValue = string | number | boolean | null | { [Key in string]?: JsonValue } | JsonValue[];

type Queries = {
    [`SELECT $1::text as message`]: {
    returnType: {
            /** 
             * Postgres data type: `TEXT`.
             */
            message: string | null,
        },
    args: [string]
},
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
    returnType: {
            /** 
             * Postgres data type: `TEXT`.
             */
            message: string | null,
            /** 
             * Postgres data type: `TEXT`.
             */
            null: string | null,
            /** 
             * Postgres data type: `BOOL`.
             */
            bool: boolean | null,
            /** 
             * Postgres data type: `INT2`.
             */
            smallint: number | null,
            /** 
             * Postgres data type: `INT4`.
             */
            integer: number | null,
            /** 
             * Postgres data type: `INT8`.
             */
            bigint: string | null,
            /** 
             * Postgres data type: `FLOAT4`.
             */
            real: number | null,
            /** 
             * Postgres data type: `FLOAT8`.
             */
            double: number | null,
            /** 
             * Postgres data type: `CHAR`.
             */
            char: string | null,
            /** 
             * Postgres data type: `TEXT`.
             */
            string: string | null,
            /** 
             * Postgres data type: `BIT`.
             */
            bit: string | null,
            /** 
             * Postgres data type: `VARBIT`.
             */
            varbit: string | null,
            /** 
             * Postgres data type: `BYTEA`.
             */
            bytea: Buffer | null,
            /** 
             * Postgres data type: `BOX`.
             */
            box: string | null,
            /** 
             * Postgres data type: `POINT`.
             */
            point: {x: number, y: number} | null,
            /** 
             * Postgres data type: `PATH`.
             */
            path: string | null,
            /** 
             * Postgres data type: `POLYGON`.
             */
            polygon: string | null,
            /** 
             * Postgres data type: `LINE`.
             */
            line: string | null,
            /** 
             * Postgres data type: `LSEG`.
             */
            lseg: string | null,
            /** 
             * Postgres data type: `CIRCLE`.
             */
            circle: {
        x: number;
        y: number;
        radius: number;
    } | null,
            /** 
             * Postgres data type: `INTERVAL`.
             */
            interval: {
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
    } | null,
            /** 
             * Postgres data type: `JSON`.
             */
            json: JsonValue | null,
            /** 
             * Postgres data type: `UUID`.
             */
            uuid: string | null,
            /** 
             * Postgres data type: `DATE`.
             */
            date: Date | null,
            /** 
             * Postgres data type: `CIDR`.
             */
            cidr: string | null,
            /** 
             * Postgres data type: `INET`.
             */
            inet: string | null,
            /** 
             * Postgres data type: `MACADDR`.
             */
            macaddr: string | null,
            /** 
             * Postgres data type: `MACADDR8`.
             */
            macaddr8: string | null,
            /** 
             * Postgres data type: `MONEY`.
             */
            numeric: string | null,
            /** 
             * Postgres data type: `NAME`.
             */
            name: string | null,
            /** 
             * Postgres data type: `OID`.
             */
            oid: number | null,
        },
    args: [string]
},

};

declare module 'pg' {
    export interface ClientBase {
    	query<T extends string>(
            q: T,
            args: T extends keyof Queries ? Queries[T]["args"] : unknown
        ): Promise<T extends keyof Queries ? QueryResult<Queries[T]["returnType"]> : unknown>;
    }
}