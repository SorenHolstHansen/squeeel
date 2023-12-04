import {
	ArrayType,
	BooleanType,
	DateType,
	NumberType,
	StringType,
} from '@squeal/core';
import { Oid } from './types';
import { ts } from 'ts-morph';
const { factory } = ts;

enum PgType {
	Bool,
	// Bytea,
	Char,
	// Name,
	Int8,
	Int2,
	Int4,
	Text,
	// Oid,
	Json,
	JsonArray,
	// Point,
	// Lseg,
	// Path,
	// Box,
	// Polygon,
	// Line,
	// LineArray,
	// Cidr,
	// CidrArray,
	// Float4,
	// Float8,
	// Unknown,
	// Circle,
	// CircleArray,
	// Macaddr8,
	// Macaddr8Array,
	// Macaddr,
	// Inet,
	// BoolArray,
	// ByteaArray,
	// CharArray,
	// NameArray,
	// Int2Array,
	// Int4Array,
	// TextArray,
	// BpcharArray,
	// VarcharArray,
	// Int8Array,
	// PointArray,
	// LsegArray,
	// PathArray,
	// BoxArray,
	// Float4Array,
	// Float8Array,
	// PolygonArray,
	// OidArray,
	// MacaddrArray,
	// InetArray,
	// Bpchar,
	Varchar,
	Date,
	Time,
	Timestamp,
	// TimestampArray,
	// DateArray,
	// TimeArray,
	Timestamptz,
	// TimestamptzArray,
	// Interval,
	// IntervalArray,
	// NumericArray,
	// Timetz,
	// TimetzArray,
	// Bit,
	// BitArray,
	// Varbit,
	// VarbitArray,
	// Numeric,
	// Record,
	// RecordArray,
	// Uuid,
	// UuidArray,
	Jsonb,
	// JsonbArray,
	// Int4Range,
	// Int4RangeArray,
	// NumRange,
	// NumRangeArray,
	// TsRange,
	// TsRangeArray,
	// TstzRange,
	// TstzRangeArray,
	// DateRange,
	// DateRangeArray,
	// Int8Range,
	// Int8RangeArray,
	// Jsonpath,
	// JsonpathArray,
	// Money,
	// MoneyArray,

	// // https://www.postgresql.org/docs/9.3/datatype-pseudo.html
	// Void,
}

export function tryGetPgTypeFromOid(oid: Oid): PgType | null {
	switch (oid) {
		case 16:
			return PgType.Bool;
		// case 17:
		// 	return PgType.Bytea;
		case 18:
			return PgType.Char;
		// case 19:
		// 	return PgType.Name;
		case 20:
			return PgType.Int8;
		case 21:
			return PgType.Int2;
		case 23:
			return PgType.Int4;
		case 25:
			return PgType.Text;
		// case 26:
		// 	return PgType.Oid;
		case 114:
			return PgType.Json;
		case 199:
			return PgType.JsonArray;
		// case 600:
		// 	return PgType.Point;
		// case 601:
		// 	return PgType.Lseg;
		// case 602:
		// 	return PgType.Path;
		// case 603:
		// 	return PgType.Box;
		// case 604:
		// 	return PgType.Polygon;
		// case 628:
		// 	return PgType.Line;
		// case 629:
		// 	return PgType.LineArray;
		// case 650:
		// 	return PgType.Cidr;
		// case 651:
		// 	return PgType.CidrArray;
		// case 700:
		// 	return PgType.Float4;
		// case 701:
		// 	return PgType.Float8;
		// case 705:
		// 	return PgType.Unknown;
		// case 718:
		// 	return PgType.Circle;
		// case 719:
		// 	return PgType.CircleArray;
		// case 774:
		// 	return PgType.Macaddr8;
		// case 775:
		// 	return PgType.Macaddr8Array;
		// case 790:
		// 	return PgType.Money;
		// case 791:
		// 	return PgType.MoneyArray;
		// case 829:
		// 	return PgType.Macaddr;
		// case 869:
		// 	return PgType.Inet;
		// case 1000:
		// 	return PgType.BoolArray;
		// case 1001:
		// 	return PgType.ByteaArray;
		// case 1002:
		// 	return PgType.CharArray;
		// case 1003:
		// 	return PgType.NameArray;
		// case 1005:
		// 	return PgType.Int2Array;
		// case 1007:
		// 	return PgType.Int4Array;
		// case 1009:
		// 	return PgType.TextArray;
		// case 1014:
		// 	return PgType.BpcharArray;
		// case 1015:
		// 	return PgType.VarcharArray;
		// case 1016:
		// 	return PgType.Int8Array;
		// case 1017:
		// 	return PgType.PointArray;
		// case 1018:
		// 	return PgType.LsegArray;
		// case 1019:
		// 	return PgType.PathArray;
		// case 1020:
		// 	return PgType.BoxArray;
		// case 1021:
		// 	return PgType.Float4Array;
		// case 1022:
		// 	return PgType.Float8Array;
		// case 1027:
		// 	return PgType.PolygonArray;
		// case 1028:
		// 	return PgType.OidArray;
		// case 1040:
		// 	return PgType.MacaddrArray;
		// case 1041:
		// 	return PgType.InetArray;
		// case 1042:
		// 	return PgType.Bpchar;
		case 1043:
			return PgType.Varchar;
		case 1082:
			return PgType.Date;
		case 1083:
			return PgType.Time;
		case 1114:
			return PgType.Timestamp;
		// case 1115:
		// 	return PgType.TimestampArray;
		// case 1182:
		// 	return PgType.DateArray;
		// case 1183:
		// 	return PgType.TimeArray;
		case 1184:
			return PgType.Timestamptz;
		// case 1185:
		// 	return PgType.TimestamptzArray;
		// case 1186:
		// 	return PgType.Interval;
		// case 1187:
		// 	return PgType.IntervalArray;
		// case 1231:
		// 	return PgType.NumericArray;
		// case 1266:
		// 	return PgType.Timetz;
		// case 1270:
		// 	return PgType.TimetzArray;
		// case 1560:
		// 	return PgType.Bit;
		// case 1561:
		// 	return PgType.BitArray;
		// case 1562:
		// 	return PgType.Varbit;
		// case 1563:
		// 	return PgType.VarbitArray;
		// case 1700:
		// 	return PgType.Numeric;
		// case 2278:
		// 	return PgType.Void;
		// case 2249:
		// 	return PgType.Record;
		// case 2287:
		// 	return PgType.RecordArray;
		// case 2950:
		// 	return PgType.Uuid;
		// case 2951:
		// 	return PgType.UuidArray;
		case 3802:
			return PgType.Jsonb;
		// case 3807:
		// 	return PgType.JsonbArray;
		// case 3904:
		// 	return PgType.Int4Range;
		// case 3905:
		// 	return PgType.Int4RangeArray;
		// case 3906:
		// 	return PgType.NumRange;
		// case 3907:
		// 	return PgType.NumRangeArray;
		// case 3908:
		// 	return PgType.TsRange;
		// case 3909:
		// 	return PgType.TsRangeArray;
		// case 3910:
		// 	return PgType.TstzRange;
		// case 3911:
		// 	return PgType.TstzRangeArray;
		// case 3912:
		// 	return PgType.DateRange;
		// case 3913:
		// 	return PgType.DateRangeArray;
		// case 3926:
		// 	return PgType.Int8Range;
		// case 3927:
		// 	return PgType.Int8RangeArray;
		// case 4072:
		// 	return PgType.Jsonpath;
		// case 4073:
		// 	return PgType.JsonpathArray;

		default:
			return null;
	}
}

export function pgTypeToTsType(pgType: PgType): ts.TypeNode {
	switch (pgType) {
		case PgType.Bool:
			return BooleanType;
		// PgType.case Bytea: return "";
		case PgType.Char:
		case PgType.Text:
		case PgType.Varchar:
			return StringType;
		// PgType.case Name: return "";
		case PgType.Int8:
		case PgType.Int2:
		case PgType.Int4:
			return NumberType;
		// PgType.case Oid: return "";
		case PgType.Json:
		case PgType.Jsonb:
			return factory.createTypeReferenceNode(
				factory.createIdentifier('JsonValue')
			);
		case PgType.JsonArray:
			return ArrayType(
				factory.createTypeReferenceNode(factory.createIdentifier('JsonValue'))
			);
		// PgType.case Point: return "";
		// PgType.case Lseg: return "";
		// PgType.case Path: return "";
		// PgType.case Box: return "";
		// PgType.case Polygon: return "";
		// PgType.case Line: return "";
		// PgType.case LineArray: return "";
		// PgType.case Cidr: return "";
		// PgType.case CidrArray: return "";
		// PgType.case Float4: return "";
		// PgType.case Float8: return "";
		// PgType.case Unknown: return "";
		// PgType.case Circle: return "";
		// PgType.case CircleArray: return "";
		// PgType.case Macaddr8: return "";
		// PgType.case Macaddr8Array: return "";
		// PgType.case Macaddr: return "";
		// PgType.case Inet: return "";
		// PgType.case BoolArray: return "";
		// PgType.case ByteaArray: return "";
		// PgType.case CharArray: return "";
		// PgType.case NameArray: return "";
		// PgType.case Int2Array: return "";
		// PgType.case Int4Array: return "";
		// PgType.case TextArray: return "";
		// PgType.case BpcharArray: return "";
		// PgType.case VarcharArray: return "";
		// PgType.case Int8Array: return "";
		// PgType.case PointArray: return "";
		// PgType.case LsegArray: return "";
		// PgType.case PathArray: return "";
		// PgType.case BoxArray: return "";
		// PgType.case Float4Array: return "";
		// PgType.case Float8Array: return "";
		// PgType.case PolygonArray: return "";
		// PgType.case OidArray: return "";
		// PgType.case MacaddrArray: return "";
		// PgType.case InetArray: return "";
		// PgType.case Bpchar: return "";
		case PgType.Date:
		case PgType.Time:
		case PgType.Timestamp:
		case PgType.Timestamptz:
			return DateType;
		// PgType.case TimestampArray: return "";
		// PgType.case DateArray: return "";
		// PgType.case TimeArray: return "";
		// PgType.case TimestamptzArray: return "";
		// PgType.case Interval: return "";
		// PgType.case IntervalArray: return "";
		// PgType.case NumericArray: return "";
		// PgType.case Timetz: return "";
		// PgType.case TimetzArray: return "";
		// PgType.case Bit: return "";
		// PgType.case BitArray: return "";
		// PgType.case Varbit: return "";
		// PgType.case VarbitArray: return "";
		// PgType.case Numeric: return "";
		// PgType.case Record: return "";
		// PgType.case RecordArray: return "";
		// PgType.case Uuid: return "";
		// PgType.case UuidArray: return "";
		// PgType.case Jsonb: return "";
		// PgType.case JsonbArray: return "";
		// PgType.case Int4Range: return "";
		// PgType.case Int4RangeArray: return "";
		// PgType.case NumRange: return "";
		// PgType.case NumRangeArray: return "";
		// PgType.case TsRange: return "";
		// PgType.case TsRangeArray: return "";
		// PgType.case TstzRange: return "";
		// PgType.case TstzRangeArray: return "";
		// PgType.case DateRange: return "";
		// PgType.case DateRangeArray: return "";
		// PgType.case Int8Range: return "";
		// PgType.case Int8RangeArray: return "";
		// PgType.case Jsonpath: return "";
		// PgType.case JsonpathArray: return "";
		// PgType.case Money: return "";
		// PgType.case MoneyArray: return "";
		// PgType.case Void: return "";
	}
}
