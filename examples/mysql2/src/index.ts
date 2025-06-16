import mysql from "mysql2/promise";

async function main() {
	// Create the connection to database
	const connection = await mysql.createConnection({
		host: "localhost",
		user: "user",
		password: "userpassword",
		database: "mydatabase",
		port: 3306,
	});

	await connection.query(`
			CREATE TABLE IF NOT EXISTS all_mysql_types (
			  id INT AUTO_INCREMENT PRIMARY KEY,

			  -- Character types
			  char_col CHAR(10),
			  varchar_col VARCHAR(255),

			  -- Binary types
			  binary_col BINARY(3),
			  varbinary_col VARBINARY(10),

			  -- Text/BLOB
			  text_col TEXT,
			  blob_col BLOB,

			  -- Integer types
			  tinyint_col TINYINT,
			  smallint_col SMALLINT,
			  mediumint_col MEDIUMINT,
			  int_col INT,
			  bigint_col BIGINT,

			  -- Float/Decimal types
			  float_col FLOAT,
			  double_col DOUBLE,
			  decimal_col DECIMAL(10,3),

			  -- Date/time types
			  date_col DATE,
			  time_col TIME,
			  datetime_col DATETIME,
			  timestamp_col TIMESTAMP,

			  -- Boolean (TINYINT used)
			  bool_col BOOLEAN,

			  -- JSON & UUID
			  json_col JSON
		);
	`);

	// 	await connection.query(`
	// 		INSERT INTO all_mysql_types (
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
		FROM all_mysql_types;
			`,
	);

	console.log(results);
	console.log(fields);

	const [results2, fields2] = await connection.query(
		"SELECT ? as input", [42]
	);

	console.log(results2);
	console.log(fields2);
}

main();
