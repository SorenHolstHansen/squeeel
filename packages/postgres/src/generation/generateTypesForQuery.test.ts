import { Client } from 'pg';
import { generateTypesForQuery } from './generateTypesForQuery';

import { beforeEach, expect, test } from 'bun:test';
import { printNode } from 'ts-morph';

const client = new Client(
	'postgres://postgres:postgres@localhost:5432/postgres'
);
await client.connect();

beforeEach(async () => {
	await client.query(`
		DROP SCHEMA public CASCADE;
		CREATE SCHEMA public;
		GRANT ALL ON SCHEMA public TO postgres;
		GRANT ALL ON SCHEMA public TO public;
`);
})

test('can generate simple integer type', async () => {
	const res = await generateTypesForQuery("SELECT 1", client);
	expect(printNode(res.outputType)).toBe(`{
    "?column?": 1;
}`);
	expect(printNode(res.argType)).toBe('[\n]');
});

test('can generate simple boolean type', async () => {
	const res = await generateTypesForQuery("SELECT true", client);
	expect(printNode(res.outputType)).toBe(`{
    "?column?": true;
}`);
	expect(printNode(res.argType)).toBe('[\n]');
});

test('can generate simple text type', async () => {
	const res = await generateTypesForQuery("SELECT 'a'", client);
	expect(printNode(res.outputType)).toBe(`{
    "?column?": "a";
}`);
	expect(printNode(res.argType)).toBe('[\n]');
});

test('can generate types for simple table query, and join', async () => {
	await client.query(`
CREATE TABLE account (
    id             SERIAL PRIMARY KEY,
    username       TEXT UNIQUE NOT NULL
);

CREATE TABLE post (
    id             SERIAL PRIMARY KEY,
    account_id     INT NOT NULL REFERENCES account(id),
    title          TEXT NOT NULL UNIQUE,
    body           TEXT,
    published      BOOLEAN,
    details        JSONB,
    likes          INT,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);
		`);
	const res1 = await generateTypesForQuery('SELECT id FROM post', client);
	expect(printNode(res1.outputType)).toBe(`{
    id: number;
}`);
	expect(printNode(res1.argType)).toBe('[\n]');

	const res2 = await generateTypesForQuery(
		'SELECT p.id, title, published, likes, created_at, details, u.username FROM post p LEFT JOIN account u ON u.id = p.account_id WHERE p.id = $1',
		client
	);
	expect(printNode(res2.outputType)).toBe(`{
    id: number;
    title: string;
    published?: boolean;
    likes?: number;
    created_at: Date;
    details?: JsonValue;
    username: string;
}`);
	expect(printNode(res2.argType)).toBe(`[
    number
]`);
});

