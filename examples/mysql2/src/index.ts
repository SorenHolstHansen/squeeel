import mysql from "mysql2/promise";

async function main() {
	// Create the connection to database
	const connection = await mysql.createConnection({
		host: "localhost",
		user: "mysql",
		password: "mysql",
		database: "squeeel",
		port: 3306,
	});

	// 	await connection.query(`
	// 		INSERT INTO a (
	//   char_col, varchar_col, binary_col, varbinary_col,
	//   text_col, blob_col, tinyint_col, smallint_col,
	//   mediumint_col, int_col, bigint_col, float_col,
	//   double_col, decimal_col, date_col, time_col,
	//   datetime_col, timestamp_col, bool_col, json_col
	// ) VALUES (
	//   'abc', 'hello world', BINARY 'xyz', 'xyz',
	//   'some text', X'DEADBEEF', 1, 123,
	//   456, 789, 123456789012345, 12.34,
	//   123456.789, 999.999, '2025-06-16', '12:34:56',
	//   '2025-06-16 12:34:56', CURRENT_TIMESTAMP, TRUE,
	//   '{"key": "value"}'
	// );
	// 	`);

	const [results, fields] = await connection.query(
		`
		SELECT
			char_col,
			varchar_col,
			bool_col,
			tinyint_col,
			smallint_col,
			mediumint_col,
			int_col,
			bigint_col,
			float_col,
			double_col,
			decimal_col,
			NULL AS null_value,
			binary_col,
			varbinary_col,
			date_col,
			time_col,
			datetime_col,
			timestamp_col,
			text_col,
			blob_col,
			json_col,
			UUID() AS uuid_col,
			INET_ATON('192.168.1.1') AS inet4_as_int_col, -- No native inet type, this gives INT representation
			CAST(123456 AS UNSIGNED) AS unsigned_int_col
		FROM a;
			`,
	);

	console.log(results);
	console.log(fields);

	const [results2, fields2] = await connection.query("SELECT ? as input", [42]);

	console.log(results2);
	console.log(fields2);
}

main();
