import { GeneratedTypes } from "../_squeal_generated_types";
import {
  Client as PgClient,
  ClientConfig,
  Pool as PgPool,
  PoolConfig,
} from "pg";

export class Client extends PgClient {
  constructor(config?: string | ClientConfig) {
    super(config);
  }

  async fetchAll<T extends string>(
    query: T,
    ...values: T extends keyof GeneratedTypes
      ? GeneratedTypes[T]["inputType"]
      : unknown[]
  ) {
    const res = await this.query(query, values);
    return res.rows as T extends keyof GeneratedTypes
      ? GeneratedTypes[T]["outputType"][]
      : unknown;
  }

  async fetchOne<T extends string>(
    query: T,
    ...values: T extends keyof GeneratedTypes
      ? GeneratedTypes[T]["inputType"]
      : unknown[]
  ) {
    const res = await this.query(query, values);
    return res.rows[0] as T extends keyof GeneratedTypes
      ? GeneratedTypes[T]["outputType"]
      : unknown;
  }
}

export class Pool extends PgPool {
  constructor(config?: PoolConfig) {
    super(config);
  }

  async fetchAll<T extends string>(
    query: T,
    ...values: T extends keyof GeneratedTypes
      ? GeneratedTypes[T]["inputType"]
      : unknown[]
  ) {
    const res = await this.query(query, values);
    return res.rows as T extends keyof GeneratedTypes
      ? GeneratedTypes[T]["outputType"][]
      : unknown;
  }

  async fetchOne<T extends string>(
    query: T,
    ...values: T extends keyof GeneratedTypes
      ? GeneratedTypes[T]["inputType"]
      : unknown[]
  ) {
    const res = await this.query(query, values);
    return res.rows[0] as T extends keyof GeneratedTypes
      ? GeneratedTypes[T]["outputType"]
      : unknown;
  }
}
