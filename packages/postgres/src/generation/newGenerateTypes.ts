import { QuestionMarkToken, UnknownType, generateId } from '@squeal/core';
import { Client } from 'pg';
import { pgTypeToTsType, tryGetPgTypeFromOid } from './typeInfo';
import { ts } from 'ts-morph';
import { Oid } from './types';
const { factory } = ts;

type PreparedStatementInfo = {
	parameter_types: Oid[];
	result_types: Oid[];
};

type Plan = {
	// e.g. "post"
	'Relation Name': string;
	// e.g. "p"
	Alias: string;
	// e.g. ['p.id', 'p.account_id', 'p.title', 'p.body', 'p.published', 'p.likes', 'p.created_at', 'p.details']
	Output: string[];
};

type QueryPlan =
	| {
			// e.g. ["p.id", "p.title", "p.published", "p.likes", "p.created_at", "p.details", "u.username"]
			Output: string[];
			Plans: Plan[];
	  }
	| Plan;

export type GeneratedTypes = {
	inputParameterType: ts.TypeNode;
	outputType: ts.TypeNode;
};

export async function newGenerateTypeForQuery(
	query: string,
	client: Client
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
	const inputParameterType = factory.createTupleTypeNode(
		parameter_types.map((t) => {
			const pgType = tryGetPgTypeFromOid(t);
			if (pgType) {
				return pgTypeToTsType(pgType);
			} else {
				return UnknownType;
			}
		})
	);

	await client.query('set plan_cache_mode = force_generic_plan;');
	const result2 = await client.query(
		`EXPLAIN (VERBOSE, FORMAT JSON) EXECUTE sample_query_${id}(NULL);`
	);
	const queryPlan: QueryPlan = result2.rows[0]['QUERY PLAN'][0]['Plan'];

	const plans = 'Plans' in queryPlan ? queryPlan.Plans : [queryPlan];
	const obj: Record<string, boolean> = {};
	await Promise.all(
		plans.map(async (plan) => {
			const b = await client.query(
				`SELECT attname, attnotnull FROM pg_attribute WHERE attrelid = '${plan['Relation Name']}'::regclass::oid`
			);
			b.rows.forEach((c) => {
				const key =
					plan.Alias === plan['Relation Name']
						? c.attname
						: `${plan.Alias}.${c.attname}`;
				obj[key] = c.attnotnull;
			});
			return obj;
		})
	);

	const typeElements: ts.PropertySignature[] = [];
	let i = 0;
	for (const field of queryPlan.Output) {
		const fieldName = field.split('.').pop()!;
		const oid = result_types[i];
		i += 1;
		const pgType = tryGetPgTypeFromOid(oid);
		if (pgType != null) {
			const typeNode = pgTypeToTsType(pgType);
			typeElements.push(
				factory.createPropertySignature(
					undefined,
					factory.createIdentifier(fieldName),
					obj[field] ? undefined : QuestionMarkToken,
					typeNode
				)
			);
		}
	}

	const outputType = factory.createTypeLiteralNode(typeElements);

	return {
		inputParameterType,
		outputType,
	};
}
