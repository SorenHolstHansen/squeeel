# Squeeel

Make your raw SQL queries type-safe instantly.

Squeeel is a CLI tool that analyzes your codebase to find all your raw SQL queries and automatically generates TypeScript types for both query parameters and return values. This means you get full type safety and autocompletion for your SQL queries without any runtime overhead or query builder complexity.

## Features

- 🔍 Automatically analyzes your codebase for SQL queries
- ✨ Generates precise TypeScript types for query parameters and return values
- 🚀 Zero runtime overhead - types are generated at build time
- 🛡️ Catches SQL errors at compile time
- 📝 Full IDE support with autocompletion and type hints
- 🔄 Works with your existing raw SQL queries - no need to change your code

## Getting started

1. Install squeeel:
```bash
npm install -D @squeeel/cli
```

2. Run the type generator:
```bash
npx @squeeel/cli gen
```

The database url, used libraries and such are automatically detected. If you need to configure anything, please run `npx @squeeel/cli gen --help` to get a list of configuration options.

That's it! Your SQL queries are now type-safe. The tool will generate a `squeeel.<lib-name>.d.ts` file with all the necessary type definitions.

## Example
 
This example is using node-postgres:

After running squeeel, a `squeeel.pg.d.ts` file will be generated, and

```typescript
const result = await client.query(
  "SELECT id, name, age FROM users WHERE age >= $1",
  // Typescript knows the types the arguments need to have, e.g. ["something"] would be an error
  [18]
);

// TypeScript knows the type of result.rows:
// { id: number, name: string, age: number }[]
```

## Supported Libraries

| Library                                                      | Considerations                                                         |
|--------------------------------------------------------------|------------------------------------------------------------------------|
| [node-postgres](https://node-postgres.com/)                  |                                                                        |
| [mysql2](https://sidorares.github.io/node-mysql2/docs)       |                                                                        |
| [better-sqlite3](https://github.com/WiseLibs/better-sqlite3) | You need to remove `@types/better-sqlite3`, we will provide the types. |

[More libraries coming soon](https://github.com/SorenHolstHansen/squeeel/issues/1)

### Unsupported Libraries

Due to TypeScript limitations with tagged templates not being generic, we cannot support:
- postgres.js
- @vercel/postgres
- bun-sql

and we also do not support any query builders.

### Why Not Query Builders?

Squeeel is designed for developers who prefer writing raw SQL queries. While query builders like Prisma and Drizzle are great tools, they:
- Add runtime overhead
- Require learning a new query syntax
- Can be less flexible than raw SQL
- Often come with their own type safety

Squeeel gives you the best of both worlds: the power and flexibility of raw SQL with the type safety of a query builder.

## Acknowledgements

We use the following other projects extensively, without which, squeeel would likely not have been possible.

- [sqlx](https://github.com/launchbadge/sqlx) - For query type inference
- [swc](https://swc.rs/) - For fast code analysis
