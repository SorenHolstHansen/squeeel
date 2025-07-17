DROP TABLE IF EXISTS a;

CREATE TABLE a (
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
