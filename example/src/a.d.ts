import type { QueryResult } from "pg";

type JsonValue = string | number | boolean | null | { [Key in string]?: JsonValue } | JsonValue[];

export type Queries = {
	'SELECT $1::text as message': {
		args: [string];
		returnType: {
            "message": string;
        };
	};
};

declare module 'pg' {
	export interface ClientBase {
		query<T extends string>(q: T, args: T extends keyof Queries ? Queries[T]["args"] : unknown): Promise<T extends keyof Queries ? QueryResult<Queries[T]["returnType"]> : unknown>;
	}
}