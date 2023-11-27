import { generateId } from '@squeal/core';
import path from 'path';
import { Client } from 'pg';
import { pgTypeToTsType, tryGetPgTypeFromOid } from './type_info';
import { Oid } from './types';

export async function generateTypesForQueries(
	queries: string[]
): Promise<void> {
	const database_url = 'postgres://postgres:postgres@localhost:5432/postgres';
	const client = new Client({ connectionString: database_url });
	await client.connect();
	const types = await Promise.all(
		queries.map(async (query) => {
			return [query, await generateTypeForQuery(query, client)];
		})
	);
	await client.end();

	let a = '{\n';
	for (const [query, _type] of types) {
		a += `\t"${query}": ${_type},\n`;
	}
	a += '}';

	Bun.write(
		path.join(__dirname, '_squeal_generated_types.ts'),
		`export type GeneratedQueryTypes = ${a};`
	);
}

function postgresTypeToTsType(oid: Oid): string {
	const pgType = tryGetPgTypeFromOid(oid);
	if (pgType != null) {
		return pgTypeToTsType(pgType);
	}
	return 'unknown';
}

async function generateTypeForQuery(
	query: string,
	client: Client
): Promise<string> {
	const id = generateId();
	const result = await client.query(`
    PREPARE sample_query_${id} AS ${query} LIMIT 0;

    CREATE TEMP TABLE tmp_sample_${id} AS EXECUTE sample_query_${id} ( NULL );

    SELECT 
      attname, 
      atttypid
    FROM pg_attribute
    WHERE attrelid = 'tmp_sample_${id}'::regclass
      AND attnum > 0
      AND NOT attisdropped
    ORDER BY attnum;
  `);
	let _type = '{\n';
	for (const row of result[2].rows) {
		_type += `\t\t${row.attname}: ${postgresTypeToTsType(row.atttypid)},\n`;
	}
	_type += '\t}';

	return _type;
}
