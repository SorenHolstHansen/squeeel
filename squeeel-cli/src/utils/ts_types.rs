use swc_atoms::Atom;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{
    Expr, Ident, TsKeywordType, TsPropertySignature, TsTupleElement, TsTupleType, TsType,
    TsTypeAnn, TsTypeElement, TsTypeLit, TsTypeRef, TsUnionOrIntersectionType, TsUnionType,
};

pub const TS_BOOLEAN_TYPE: TsType = TsType::TsKeywordType(TsKeywordType {
    span: DUMMY_SP,
    kind: swc_ecma_ast::TsKeywordTypeKind::TsBooleanKeyword,
});

pub const TS_NUMBER_TYPE: TsType = TsType::TsKeywordType(TsKeywordType {
    span: DUMMY_SP,
    kind: swc_ecma_ast::TsKeywordTypeKind::TsNumberKeyword,
});

pub const TS_STRING_TYPE: TsType = TsType::TsKeywordType(TsKeywordType {
    span: DUMMY_SP,
    kind: swc_ecma_ast::TsKeywordTypeKind::TsStringKeyword,
});

pub const TS_NULL_TYPE: TsType = TsType::TsKeywordType(TsKeywordType {
    span: DUMMY_SP,
    kind: swc_ecma_ast::TsKeywordTypeKind::TsNullKeyword,
});

pub const TS_NEVER_TYPE: TsType = TsType::TsKeywordType(TsKeywordType {
    span: DUMMY_SP,
    kind: swc_ecma_ast::TsKeywordTypeKind::TsNeverKeyword,
});

pub const TS_UNDEFINED_TYPE: TsType = TsType::TsKeywordType(TsKeywordType {
    span: DUMMY_SP,
    kind: swc_ecma_ast::TsKeywordTypeKind::TsUndefinedKeyword,
});

pub const TS_ANY_TYPE: TsType = TsType::TsKeywordType(TsKeywordType {
    span: DUMMY_SP,
    kind: swc_ecma_ast::TsKeywordTypeKind::TsAnyKeyword,
});

pub const TS_BIGINT_TYPE: TsType = TsType::TsKeywordType(TsKeywordType {
    span: DUMMY_SP,
    kind: swc_ecma_ast::TsKeywordTypeKind::TsBigIntKeyword,
});

pub const TS_UNKNOWN_TYPE: TsType = TsType::TsKeywordType(TsKeywordType {
    span: DUMMY_SP,
    kind: swc_ecma_ast::TsKeywordTypeKind::TsUnknownKeyword,
});

pub fn ts_type_ref<Sym: Into<Atom>>(sym: Sym) -> TsType {
    TsType::TsTypeRef(TsTypeRef {
        span: DUMMY_SP,
        type_name: swc_ecma_ast::TsEntityName::Ident(Ident::new_no_ctxt(sym.into(), DUMMY_SP)),
        type_params: None,
    })
}

pub fn ts_object_type<Members: IntoIterator<Item = (Expr, TsType, bool)>>(
    members: Members,
) -> TsType {
    TsType::TsTypeLit(TsTypeLit {
        span: DUMMY_SP,
        members: members
            .into_iter()
            .map(|(name, ty, optional)| {
                TsTypeElement::TsPropertySignature(TsPropertySignature {
                    span: DUMMY_SP,
                    key: Box::new(name),
                    type_ann: Some(Box::new(TsTypeAnn {
                        span: DUMMY_SP,
                        type_ann: Box::new(ty),
                    })),
                    optional,
                    readonly: false,
                    computed: false,
                })
            })
            .collect(),
    })
}

pub fn ts_object_type_computed<Members: IntoIterator<Item = (Expr, TsType, bool)>>(
    members: Members,
) -> TsType {
    TsType::TsTypeLit(TsTypeLit {
        span: DUMMY_SP,
        members: members
            .into_iter()
            .map(|(name, ty, optional)| {
                TsTypeElement::TsPropertySignature(TsPropertySignature {
                    span: DUMMY_SP,
                    key: Box::new(name),
                    type_ann: Some(Box::new(TsTypeAnn {
                        span: DUMMY_SP,
                        type_ann: Box::new(ty),
                    })),
                    optional,
                    readonly: false,
                    computed: true,
                })
            })
            .collect(),
    })
}

pub fn ts_nullable_type(ty: TsType) -> TsType {
    TsType::TsUnionOrIntersectionType(TsUnionOrIntersectionType::TsUnionType(TsUnionType {
        span: DUMMY_SP,
        types: vec![Box::new(ty), Box::new(TS_NULL_TYPE)],
    }))
}

pub fn ts_optional_type(ty: TsType) -> TsType {
    TsType::TsUnionOrIntersectionType(TsUnionOrIntersectionType::TsUnionType(TsUnionType {
        span: DUMMY_SP,
        types: vec![Box::new(ty), Box::new(TS_UNDEFINED_TYPE)],
    }))
}

pub fn ts_tuple_type<ElemTypes: IntoIterator<Item = TsType>>(types: ElemTypes) -> TsType {
    TsType::TsTupleType(TsTupleType {
        span: DUMMY_SP,
        elem_types: types
            .into_iter()
            .map(|ty| TsTupleElement {
                span: DUMMY_SP,
                label: None,
                ty: Box::new(ty),
            })
            .collect(),
    })
}
