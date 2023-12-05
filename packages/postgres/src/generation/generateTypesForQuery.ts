import { QuestionMarkToken, UnknownType, generateId } from "@squeal/core";
import { Client } from "pg";
import { pgTypeToTsType, tryGetPgTypeFromOid } from "./typeInfo";
import { ts } from "ts-morph";
import { Oid } from "./types";
const { factory } = ts;

function isNumeric(str: string): boolean {
  if (typeof str != "string") return false; // we only process strings!
  return (
    !isNaN(str as any) && // use type coercion to parse the _entirety_ of the string (`parseFloat` alone does not do this)...
    !isNaN(parseFloat(str))
  ); // ...and ensure strings of whitespace fail
}

type PreparedStatementInfo = {
  parameter_types: Oid[];
  result_types: Oid[];
};

// Very basic case. Just something like SELECT 1, 2
type ResultQueryPlan = {
  // e.g. ["1"] for things like SELECT 1
  "Node Type": "Result";
  Output: string[];
};

type QueryPlan =
  | {
    "Node Type": string;
    // e.g. "post"
    "Relation Name": string;
    // e.g. "public"
    Schema: string;
    // e.g. "post" or "p"
    Alias: string;
    // e.g. ["id"] or with alias like ["p.id"]
    Output: string[];
  }
  | {
    // e.g. ["p.id", "p.title", "p.likes", "u.username"]
    "Node Type": string;
    Output: string[];
    Plans: {
      // e.g. "post"
      "Relation Name": string;
      // e.g. "public"
      Schema: string;
      // e.g. "p" or "post"
      Alias: string;
      // e.g. ["p.id", "p.title", "p.likes"]
      Output: string[];
    }[];
  }
  | ResultQueryPlan;

export type GeneratedTypes = {
  argType: ts.TypeNode;
  outputType: ts.TypeNode;
};

export async function generateTypesForQuery(
  query: string,
  enums: Record<Oid, ts.UnionTypeNode>,
  client: Client,
): Promise<GeneratedTypes> {
  const id = generateId();
  await client.query(`PREPARE sample_query_${id} AS ${query}`);
  const result = await client.query(`
        SELECT 
            name, 
            statement, 
            prepare_time, 
            parameter_types::oid[], 
            result_types::oid[]
        FROM pg_prepared_statements 
        WHERE name = 'sample_query_${id}';`);
  const { parameter_types, result_types }: PreparedStatementInfo =
    result.rows[0];
  const argType = factory.createTupleTypeNode(
    parameter_types.map((t) => {
      const pgType = tryGetPgTypeFromOid(t);
      if (pgType) {
        return pgTypeToTsType(pgType);
      } else {
        return UnknownType;
      }
    }),
  );

  await client.query("set plan_cache_mode = force_generic_plan;");
  const result2 = await client.query(
    `EXPLAIN (VERBOSE, FORMAT JSON) EXECUTE sample_query_${id}(NULL);`,
  );
  const queryPlan: QueryPlan = result2.rows[0]["QUERY PLAN"][0]["Plan"];
  if (queryPlan["Node Type"] === "Result") {
    // Really simple case, just something like select 1, 'a'
    // Since objects can't have duplicate field names, this will result in the last output only
    const lastOutput = queryPlan.Output.at(-1);
    if (lastOutput == null) throw new Error("Found a query with no output");
    let node:
      | ts.LiteralExpression
      | ts.NullLiteral
      | ts.BooleanLiteral
      | ts.PrefixUnaryExpression = factory.createStringLiteral(lastOutput);
    if (isNumeric(lastOutput)) {
      node = factory.createNumericLiteral(lastOutput);
    } else if (lastOutput === "true") {
      node = factory.createTrue();
    } else if (lastOutput === "false") {
      node = factory.createFalse();
    } else {
      node = factory.createStringLiteral(
        lastOutput.replace("::text", "").replace(/'/g, ""),
      );
    }
    const outputType = factory.createTypeLiteralNode([
      factory.createPropertySignature(
        undefined,
        factory.createStringLiteral("?column?"),
        undefined,
        factory.createLiteralTypeNode(node),
      ),
    ]);

    return { argType, outputType };
  } else if ("Plans" in queryPlan) {
    const plans = queryPlan.Plans;
    const obj: Record<string, boolean> = {};
    await Promise.all(
      plans.map(async (plan) => {
        const b = await client.query(
          `SELECT attname, attnotnull FROM pg_attribute WHERE attrelid = '${plan["Relation Name"]}'::regclass::oid`,
        );
        b.rows.forEach((c) => {
          const key =
            plan.Alias === plan["Relation Name"]
              ? c.attname
              : `${plan.Alias}.${c.attname}`;
          obj[key] = c.attnotnull;
        });
        return obj;
      }),
    );

    const typeElements: ts.PropertySignature[] = [];
    let i = 0;
    for (const field of queryPlan.Output) {
      const fieldName = field.split(".").pop()!;
      const oid = result_types[i];
      i += 1;
      const pgType = tryGetPgTypeFromOid(oid);
      const typeNode = pgType != null ? pgTypeToTsType(pgType) : enums[oid];
      if (typeNode != null) {
        typeElements.push(
          factory.createPropertySignature(
            undefined,
            factory.createIdentifier(fieldName),
            obj[field] ? undefined : QuestionMarkToken,
            typeNode,
          ),
        );
      }
    }

    const outputType = factory.createTypeLiteralNode(typeElements);

    return {
      argType,
      outputType,
    };
  } else if ("Relation Name" in queryPlan) {
    const plans = [queryPlan];
    const obj: Record<string, boolean> = {};
    await Promise.all(
      plans.map(async (plan) => {
        const b = await client.query(
          `SELECT attname, attnotnull FROM pg_attribute WHERE attrelid = '${plan["Relation Name"]}'::regclass::oid`,
        );
        b.rows.forEach((c) => {
          const key =
            plan.Alias === plan["Relation Name"]
              ? c.attname
              : `${plan.Alias}.${c.attname}`;
          obj[key] = c.attnotnull;
        });
        return obj;
      }),
    );

    const typeElements: ts.PropertySignature[] = [];
    let i = 0;
    for (const field of queryPlan.Output) {
      const fieldName = field.split(".").pop()!;
      const oid = result_types[i];
      i += 1;
      const pgType = tryGetPgTypeFromOid(oid);
      const typeNode = pgType != null ? pgTypeToTsType(pgType) : enums[oid];
      if (typeNode != null) {
        typeElements.push(
          factory.createPropertySignature(
            undefined,
            factory.createIdentifier(fieldName),
            obj[field] ? undefined : QuestionMarkToken,
            typeNode,
          ),
        );
      }
    }

    const outputType = factory.createTypeLiteralNode(typeElements);

    return {
      argType,
      outputType,
    };
  }

  throw new Error("Unhandled query type");
}
