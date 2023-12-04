import { Client } from 'pg';
import { generateTypesForQuery } from './generateTypesForQuery';

import { expect, test } from 'bun:test';
import { printNode } from 'ts-morph';

const client = new Client(
	'postgres://postgres:postgres@localhost:5432/postgres'
);
await client.connect();

test('simple output type', async () => {
	const a = await generateTypesForQuery('SELECT id FROM post', client);
	expect(printNode(a.outputType)).toBe(`{
    id: number;
}`);
	expect(printNode(a.argType)).toBe('[\n]');
});

test('simple output type2', async () => {
	const a = await generateTypesForQuery(
		'SELECT p.id, title, published, likes, created_at, details, u.username FROM post p LEFT JOIN account u ON u.id = p.account_id WHERE p.id = $1',
		client
	);
	expect(printNode(a.outputType)).toBe(`{
    id: number;
    title: string;
    published?: boolean;
    likes?: number;
    created_at: Date;
    details?: JsonValue;
    username: string;
}`);
	expect(printNode(a.argType)).toBe(`[
    number
]`);
});

test('simple output type 3', async () => {
	console.log((await client.query("SELECT 1, 'a'")).rows);
	const res = await generateTypesForQuery("SELECT 1, 'a'", client);
	expect(printNode(res.outputType)).toBe(`{
    "?column?": "a";
}`);
	expect(printNode(res.argType)).toBe('[\n]');
});

