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

That's it! Your SQL queries are now type-safe. The tool will generate a `squeeel.d.ts` file with all the necessary type definitions.

## Example

```typescript
// Your SQL query
const result = await client.query(
  `SELECT id, name, email FROM users WHERE age > $1`,
  [18]
);

// TypeScript now knows exactly what result.rows contains:
// Array<{ id: number, name: string, email: string }>
```

## Supported Libraries

- [node-postgres](https://node-postgres.com/)
- [better-sqlite3](https://github.com/WiseLibs/better-sqlite3)

[More libraries coming soon](https://github.com/SorenHolstHansen/squeeel/issues/1)

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

- [sqlx](https://github.com/launchbadge/sqlx) - Inspired by Rust's sqlx
- [swc](https://swc.rs/) - For fast code analysis