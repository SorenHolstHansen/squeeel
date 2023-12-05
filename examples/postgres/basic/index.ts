import { q } from '@squeeel/postgres' with { type: 'macro' };
import { Client } from '@squeeel/postgres';

const client = new Client("postgres://postgres:postgres@localhost:5432/postgres");
await client.connect();
const res = await client.fetchAll(q("SELECT p.id, title, published, likes, details, created_at, u.username FROM post p LEFT JOIN account u ON u.id = p.account_id WHERE p.id = $1"), 1);
console.log({ res })
const res2 = await client.fetchOne(q("SELECT id FROM post"));
await client.end();
