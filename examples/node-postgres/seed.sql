DROP TABLE IF EXISTS a;
DROP TYPE IF EXISTS my_enum;

CREATE TYPE my_enum AS ENUM ('a', 'b', 'c');


CREATE TABLE a (
    id SERIAL PRIMARY KEY,
	b BOOL,
	si SMALLINT,
	i INTEGER,
	bi BIGINT,
	r REAL,
	d DOUBLE PRECISION,
	c CHAR,
	s TEXT,
	bt BIT,
	vb VARBIT,
	bta BYTEA,
	bx BOX,
	pnt POINT,
	pth PATH,
	plgn POLYGON,
	ln LINE,
	lsg LSEG,
	crcl CIRCLE,
	intvl INTERVAL,
	jsn JSON,
	uid UUID,
	dt DATE,
	cdr CIDR,
	nt INET,
	mcddr MACADDR,
	mcd8 MACADDR8,
	num NUMERIC,
	nm NAME,
	en my_enum
);
