import { q } from '@squeal/postgres' with { type: 'macro' };
import { Client } from '@squeal/postgres';

const client = new Client("postgres://postgres:postgres@localhost:5432/postgres");
await client.connect();
const res = await client.fetchAll(q("SELECT * FROM post"));
console.log({res})
const res2 = await client.fetchOne(q("SELECT id FROM post"));
await client.end();
