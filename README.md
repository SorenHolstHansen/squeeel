# squeal

POC of a compile time type generating sql client.

## Usage (Postgres)

The postgres client is basically a thin wrapper around the [`pg`](https://www.npmjs.com/package/pg) package, with only two added methods. So you should read their documentation as well.

Assuming you have setup your database, and ran a migration like this

```sql
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
    likes          INT,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

You will get build-time generated queries like this. That is, you write the following, and after running `bun build <file>` you will get the types in the comments

```ts
import { q } from '@squeal/postgres' with { type: 'macro' };
import { Client } from '@squeal/postgres'; // PgClient is a wrapper around Client

const client = new Client();
await client.connect();
const res = await client.fetchAll(q("SELECT * FROM post"));
// { id: number, account_id: number, title: string, body?: string, published?: boolean, likes?: number, created_at: Date}

// to only get the first result do
// const res = await client.fetchOne("SELECT * FROM post WHERE id = 1");
await client.end();
```

## Examples

Try going into the examples/postgres/basic, start a postgres database, and run the migration in the migration file. After that, run `bun build index.ts --target bun`, then (possibly after restarting your lsp), you should now have the right types for the query results.
