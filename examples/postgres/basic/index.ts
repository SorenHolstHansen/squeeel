import { q } from '@squeal/postgres' with { type: 'macro' };
import { PgClient } from '@squeal/postgres';

const client = new PgClient("postgres://postgres:postgres@localhost:5432/postgres");
await client.connect();
const res = await client.query(q("SELECT * FROM post"));
console.log({res})
const res2 = await client.query(q("SELECT id FROM post"));
await client.end();
