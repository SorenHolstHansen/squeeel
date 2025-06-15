import { Client } from "pg";

async function main() {
	const client = new Client("postgres://postgres:postgres@localhost:5432/postgres");
	await client.connect();
	const res = await client.query("SELECT $1::text as message", ["1"]);
	console.log("Simple", res.rows[0].message);
	await client.end();
}

main()
