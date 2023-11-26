# squeal

Squeal is a typescript + bun SQL package featuring build-time checked queries and type generation without a DSL.

The js/ts ecosystem is filled with ORMs offering their own custom DSL to make SQL "nicer" to work with. ORMs makes the easy things simply but the hard things impossible. They often don't have support for the whole SQL standard, so you anyway have to do raw sql statements for some things, for instance when you need performance, which the ORMs often also lack.

The DSLs also often means learning a new language and how that language maps to the sql underneath the hood, when, in my opinion, SQL is an amazing language. Why not just use it?

Squeal offers a solution to this. You write pure sql, and we generate the times for you. At build-time that is. This was previously not possibly, but now with the macro system in bun, it is. This does need that Node and Deno is not supported for the moment, but we do have a feature on the roadmap, that will generate the types when running a generator.


## Installation 

To install run
```bash
npm install @squeal/postgres
```

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

You will get build-time generated queries like this
```ts
import { q } from '@squeal/postgres' with { type: 'macro' };
import { PgClient } from '@squeal/postgres'; // PgClient is a wrapper around Client

const client = new PgClient();
await client.connect();
const res = await client.fetchAll(q("SELECT * FROM post"));
// { id: number, account_id: number, title: string, body?: string, published?: boolean, likes?: number, created_at: Date}

// to only get the first result do
// const res = await client.fetchOne("SELECT * FROM post WHERE id = 1");
await client.end();
```

## Current limitations and unsupported features

- As macros are only supported in bun, we do not support build-time type generation in Node or Deno. We have a feature on the roadmap to create a cli generator that can be run whenever, so you can get the same benefits, just not automatically
- Migration tool. We currently do not offer a migration tool. There are plenty of great options out there to choose from.

