import Database from "better-sqlite3";
const db = new Database("example.db");

async function main() {
	const results = db
		.prepare(
			`
			SELECT 
				NULL AS n,
				1 AS int,
				1.5 AS float,
				'hello' AS string,
				x'0010' AS buffer,
				true AS bool
        	`,
		)
		.get();
	console.log(results);

	const results2 = db
		.prepare(
			"SELECT ?"
		)
		.get(1);
	console.log(results2);
}

main();
