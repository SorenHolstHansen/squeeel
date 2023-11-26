import { GeneratedQueryTypes } from './_squeal_generated_types';
import { Client, ClientConfig } from 'pg';

export class PgClient {
  pg_client: Client;
  constructor(config?: string | ClientConfig) {
    this.pg_client = new Client(config);
  }

  async connect() {
    return await this.pg_client.connect();
  }

  async end() {
    return await this.pg_client.end();
  }

  async query<T extends string>(query: T, values?: string[]) {
    const res = await this.pg_client.query(query, values);
    return res.rows as T extends keyof GeneratedQueryTypes ? GeneratedQueryTypes[T][] : unknown;
  }
}

// export async function perform<T extends string>(query: T, client: Client) {
//   const res = await client.query(query);
//   return query as T extends keyof GeneratedQueryTypes ? GeneratedQueryTypes[T] : unknown;
// }
