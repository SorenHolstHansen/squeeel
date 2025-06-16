import Database from "better-sqlite3";
const db = new Database("example.db");

async function main() {
	const res = db
		.prepare(
			`
        SELECT 
			NULL AS n,
			1 AS int,
			1.5 AS float,
			'hello' AS string,
			x'0010' AS buffer,
			true AS bool,
			? AS f
        `,
		)
		.get(1);
	console.log("res:", res);
}

main();
