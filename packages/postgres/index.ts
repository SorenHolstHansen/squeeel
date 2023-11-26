import { Loader } from '@squeal/core'
import path from 'path';
export * from './query';
import { Client } from 'pg';

async function generateTypesForQueries(queries: string[]): Promise<void> {
  console.log(`Generating types for ${queries.length} queries!`);
  const database_url = "postgres://postgres:postgres@localhost:5432/postgres";
  const client = new Client({ connectionString: database_url });
  await client.connect();
  const types = await Promise.all(queries.map(async (query) => {
    return [query, await generateTypeForQuery(query, client)]
  }));
  await client.end();

  let a = "{\n";
  for (const [query, _type] of types) {
    a += `\t"${query}": ${_type},\n`;
  }
  a += "}";

  Bun.write(path.join(__dirname, "_squeal_generated_types.ts"), `export type GeneratedQueryTypes = ${a};`);
}

const loader = new Loader(generateTypesForQueries);

export function q<T extends string>(query: T): T {
  loader.load(query);

  return query;
}

function postgresTypeToTsType(type: string): string {
  switch (type) {
    case "integer": return "number";
    case "text": return "string";
    case "boolean": return "boolean";
    case "timestamp with time zone": return "Date";
    default: return type
  }
}

let id = 0;
function generateId(): number {
  return id++;
}

async function generateTypeForQuery(query: string, client: Client): Promise<string> {
  const id = generateId();
  const result = await client.query(`
    PREPARE sample_query_${id} AS ${query} LIMIT 0;

    CREATE TEMP TABLE tmp_sample_${id} AS EXECUTE sample_query_${id} ( NULL );

    SELECT attname, format_type(atttypid, atttypmod) AS type
    FROM pg_attribute
    WHERE attrelid = 'tmp_sample_${id}'::regclass
      AND attnum > 0
      AND NOT attisdropped
    ORDER BY attnum;
  `);
  let _type = "{\n";
  for (const row of result[2].rows) {
    _type += `\t\t${row.attname}: ${postgresTypeToTsType(row.type)},\n`;
  }
  _type += "\t}";

  return _type;
}

