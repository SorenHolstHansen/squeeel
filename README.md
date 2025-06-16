# Squeeel

Instantly make your database queries typesafe.

## Getting started

In your repository, just run
```
npx @squeeel/cli gen
```
And your database queries should be typesafe.

## Supported libraries

- [node-postgres](https://node-postgres.com/)
- [better-sqlite3](https://github.com/WiseLibs/better-sqlite3)

With [more coming](https://github.com/SorenHolstHansen/squeeel/issues/1)

### Unsupported libraries

We won't support any sql libraries that are primarily query builders, i.e. prisma, drizzle and so on, since squeeel works by infering the types from raw sql queries, and since the libraries themselves often come with typesafety built-in.

However there are also some sql libraries using raw sql queries that we also probably won't ever support.
The main reasons are that [tagged templates aren't generic](https://github.com/microsoft/TypeScript/issues/33304).

- postgres.js
- @vercel/postgres
- bun-sql
