import path from 'path';
export * from './query';
import { Client } from 'pg';


class Loader {
  queries: string[] = [];
  batchLoaderHasBeenCalled = false;

  async generateTypes(queries: string[]) {
    console.log(`Generating types for ${queries.length} queries`);
  }

  async load(query: string) {
    return new Promise(async (resolve, reject) => {
      this.queries.push(query);
      process.nextTick(async () => {
        if (!this.batchLoaderHasBeenCalled) {
          this.generateTypes(this.queries);
          this.batchLoaderHasBeenCalled = true;
        }
        resolve(undefined)
      });
    });
  }
}

const loader = new Loader();

export function q<T extends string>(query: T): T {
  loader.load(query);

  return query;
}

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

function postgresTypeToTsType(type: string): string {
  switch (type) {
    case "integer": return "number";
    case "text": return "string";
    case "boolean": return "boolean";
    case "timestamp with time zone": return "string";
    default: return type
  }
}
