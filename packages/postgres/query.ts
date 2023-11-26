import { GeneratedQueryTypes } from "./_squeal_generated_types";
import { Client, ClientConfig, Pool, PoolConfig } from "pg";

export class PgClient extends Client {
  constructor(config?: string | ClientConfig) {
    super(config);
  }

  async fetchAll<T extends string>(query: T, values?: string[]) {
    const res = await this.query(query, values);
    return res.rows as T extends keyof GeneratedQueryTypes
      ? GeneratedQueryTypes[T][]
      : unknown;
  }

  async fetchOne<T extends string>(query: T, values?: string[]) {
    const res = await this.query(query, values);
    return res.rows[0] as T extends keyof GeneratedQueryTypes
      ? GeneratedQueryTypes[T]
      : unknown;
  }
}

export class PgPool extends Pool {
  constructor(config?: PoolConfig) {
    super(config);
  }

  async fetchAll<T extends string>(query: T, values?: string[]) {
    const res = await this.query(query, values);
    return res.rows as T extends keyof GeneratedQueryTypes
      ? GeneratedQueryTypes[T][]
      : unknown;
  }

  async fetchOne<T extends string>(query: T, values?: string[]) {
    const res = await this.query(query, values);
    return res.rows[0] as T extends keyof GeneratedQueryTypes
      ? GeneratedQueryTypes[T]
      : unknown;
  }
}
