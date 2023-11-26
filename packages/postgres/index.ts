import { Loader } from '@squeal/core'
import path from 'path';
export * from './query';

async function generateTypeForQuery(query: string): Promise<void> {
  // const database_url = "postgres://postgres:postgres@localhost:5432/postgres";
  // const client = new Client({ connectionString: database_url });
  // await client.connect();
  // console.log("In here");
  // const result = await client.query(`
  //   PREPARE sample_query AS ${query} LIMIT 0;
  //   CREATE TEMP TABLE tmp_sample AS EXECUTE sample_query ( NULL );
  //   SELECT attname, format_type(atttypid, atttypmod) AS type
  //   FROM pg_attribute
  //   WHERE attrelid = 'tmp_sample'::regclass
  //     AND attnum > 0
  //     AND NOT attisdropped
  //   ORDER BY attnum;
  // `);
  // let _type = "{";
  // for (const row of result[2].rows) {
  //   _type += `${row.attname}: ${postgresTypeToTsType(row.type)},`;
  // }
  // _type += "}";
  // console.log(_type)

  // Bun.write(path.join(__dirname, "_squeal_generated_types.ts"), `export type MyType = {"${query}": ${_type}};`);

  // await client.end();
}

async function generateTypesForQueries(queries: string[]): Promise<void> {
  console.log({queries})
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
    case "timestamp with time zone": return "string";
    default: return type
  }
}
