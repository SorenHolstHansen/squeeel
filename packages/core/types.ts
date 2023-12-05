import { ts } from 'ts-morph';
const { factory } = ts;

export const ExportKeyword = factory.createToken(ts.SyntaxKind.ExportKeyword);
export const StringType = factory.createKeywordTypeNode(
	ts.SyntaxKind.StringKeyword
);
export const NumberType = factory.createKeywordTypeNode(
	ts.SyntaxKind.NumberKeyword
);
export const BooleanType = factory.createKeywordTypeNode(
	ts.SyntaxKind.BooleanKeyword
);
export const NullType = factory.createLiteralTypeNode(factory.createNull());
export const ArrayType = (subType: ts.TypeNode) => {
	return factory.createArrayTypeNode(subType);
};
export const DateType = factory.createTypeReferenceNode(
	factory.createIdentifier('Date')
);
export const QuestionMarkToken = factory.createToken(
	ts.SyntaxKind.QuestionToken
);
export const UnknownType = factory.createKeywordTypeNode(
	ts.SyntaxKind.UnknownKeyword
);
export const JsonType = factory.createTypeReferenceNode(
	factory.createIdentifier('JsonValue')
);

export const JsonValueType = factory.createTypeAliasDeclaration(
	[ExportKeyword],
	factory.createIdentifier('JsonValue'),
	undefined,
	factory.createUnionTypeNode([
		StringType,
		NumberType,
		BooleanType,
		NullType,
		ArrayType(
			factory.createTypeReferenceNode(
				factory.createIdentifier('JsonValue'),
				undefined
			)
		),
		factory.createTypeLiteralNode([
			factory.createIndexSignature(
				undefined,
				[
					factory.createParameterDeclaration(
						undefined,
						undefined,
						factory.createIdentifier('key'),
						undefined,
						StringType,
						undefined
					),
				],
				factory.createTypeReferenceNode(
					factory.createIdentifier('JsonValue'),
					undefined
				)
			),
		]),
	])
);
