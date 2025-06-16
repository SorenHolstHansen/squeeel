import { Client } from "pg";

async function main() {
	const client = new Client("postgres://postgres:postgres@localhost:5432/postgres");
	await client.connect();

	const results = await client.query(
		`
        SELECT 
			NULL as null,
			true as bool,
			1::smallint as smallint,
			1::integer as integer,
			1::bigint as bigint,
			1.5::real as real,
			1.5::double precision as double,
			'c'::char as char,
			'hello' as string,
			x'0010' as bit,
			x'0010'::varbit as varbit,
			'\xDEADBEEF'::bytea as bytea,
			'(2,2),(0,0)'::box as box,
			POINT(1.2, 123.1) as point,
			'[(0,0),(1,1)]'::path as path,
			'((0,0),(1,1))'::polygon as polygon,
			LINE(POINT(1.2, 123.1), POINT(123.1, 1.2)) as line,
			LSEG(POINT(1.2, 123.1), POINT(123.1, 1.2)) as lseg,
			CIRCLE(POINT(1.2, 123.1), 10) as circle,
			'1 year 2 months 2 days 3 hours 20 minutes 1 second 20 milliseconds'::interval as interval,
			'{"hello": "world"}'::json as json,
			gen_random_uuid() as uuid,
			'2001-09-28'::date as date,
			'10.1.0.0/16'::cidr as cidr,
			'10.1.0.0/16'::inet as inet,
			'08:00:2b:01:02:03'::macaddr as macaddr,
			'08:00:2b:01:02:03:04:05'::macaddr8 as macaddr8,
			'12.34'::float8::numeric::money as numeric,
			'matt'::name as name,
			1::oid
        `
	);
    console.log(results);

	const results2 = await client.query(
		"SELECT $1", ["Hello, world"]
	);
    console.log(results2);

    await client.end();
}

main()