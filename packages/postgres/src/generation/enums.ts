import { Client } from "pg";
import { Oid } from "./types";
import { ts } from "ts-morph";
const { factory } = ts;

export async function generateTypesForEnums(client: Client): Promise<Record<Oid, ts.UnionTypeNode>> {
	const res = await client.query("SELECT enumtypid, ARRAY_AGG(enumlabel) AS vals FROM pg_enum GROUP BY enumtypid;");

	const enums: Record<Oid, ts.UnionTypeNode> = {};

	for (const row of res.rows) {
		const values = (row.vals as string).replace(/^{/, "").replace(/}$/, "").split(",");
		const type = factory.createUnionTypeNode(values.map((val: string) => (
			factory.createLiteralTypeNode(factory.createStringLiteral(val))

		)));
		enums[row["enumtypid"]] = type;
	}

	return enums;
}
