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

The database url, used libraries and such are automatically detected. If you need to configure anything, please see [the configuration section](#configuration)

That's it! Your SQL queries are now type-safe. The tool will generate a `squeeel.d.ts` file with all the necessary type definitions.

## Example
 
This example is using node-postgres:

After running squeeel

```typescript
const result = await client.query(
  "SELECT id, name, age FROM users WHERE age >= $1",
  // Typescript knows that this must be a number
  [18]
);

// TypeScript nows exactly the type of result.rows:
// { id: number, name: string, age: number }[]
```

## Supported Libraries

| Library                                                      | Considerations                                                         |
|--------------------------------------------------------------|------------------------------------------------------------------------|
| [node-postgres](https://node-postgres.com/)                  |                                                                        |
| [mysql2](https://sidorares.github.io/node-mysql2/docs)       |                                                                        |
| [better-sqlite3](https://github.com/WiseLibs/better-sqlite3) | You need to remove `@types/better-sqlite3`, we will provide the types. |

[More libraries coming soon](https://github.com/SorenHolstHansen/squeeel/issues/1)

## Configuration

We are currently working on the configuration setup.

### Why Not Query Builders?

Squeeel is designed for developers who prefer writing raw SQL queries. While query builders like Prisma and Drizzle are great tools, they:
- Add runtime overhead
- Require learning a new query syntax
- Can be less flexible than raw SQL
- Often come with their own type safety

Squeeel gives you the best of both worlds: the power and flexibility of raw SQL with the type safety of a query builder.

### Unsupported Libraries

Due to TypeScript limitations with tagged templates, we cannot support:
- postgres.js
- @vercel/postgres
- bun-sql

## Acknowledgements

We use the following other projects extensively, without which, squeeel would likely not have been possible.

- [sqlx](https://github.com/launchbadge/sqlx) - Inspired by Rust's sqlx
- [swc](https://swc.rs/) - For fast code analysis
