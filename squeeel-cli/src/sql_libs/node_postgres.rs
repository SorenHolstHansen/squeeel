use super::SupportedLib;
use crate::utils::ts_types::{
    TS_BOOLEAN_TYPE, TS_NULL_TYPE, TS_NUMBER_TYPE, TS_STRING_TYPE, TS_UNKNOWN_TYPE, ts_lit_type,
    ts_object_type, ts_type_ref,
};
use crate::{sql_libs::SqlLib, visitor::Query};
use sqlx_core::type_info::TypeInfo;
use swc_common::Span;
use swc_ecma_ast::{
    BindingIdent, Decl, ExportDecl, ImportDecl, ImportNamedSpecifier, ImportPhase, ImportSpecifier,
    ModuleDecl, ModuleItem, Stmt, Str, TruePlusMinus, TsArrayType, TsConditionalType, TsEntityName,
    TsFnParam, TsIndexedAccessType, TsInterfaceBody, TsInterfaceDecl, 
    TsMappedType, TsMethodSignature, TsModuleBlock, TsModuleDecl, TsModuleName, TsNamespaceBody,
    TsType, TsTypeAliasDecl, TsTypeAnn, TsTypeElement, TsTypeOperator, TsTypeOperatorOp,
    TsTypeParam, TsTypeParamDecl, TsTypeParamInstantiation, TsTypeRef, TsUnionOrIntersectionType,
    TsUnionType,
};

pub struct NodePostgres;

impl SqlLib for NodePostgres {
    type Db = sqlx::Postgres;

    fn parse_call_expr(&self, call_expr: &swc_ecma_ast::CallExpr) -> Option<crate::visitor::Query> {
        let swc_ecma_ast::Callee::Expr(expr) = &call_expr.callee else {
            return None;
        };

        let swc_ecma_ast::Expr::Member(member_expr) = &**expr else {
            return None;
        };

        let obj = &member_expr.obj.as_ident()?.sym;
        let prop = &member_expr.prop.as_ident()?.sym;
        if obj != "client" || prop != "query" {
            return None;
        }

        let mut args_iter = call_expr.args.iter();
        let query_expr = args_iter.next()?;
        let _args = args_iter.next();
        if args_iter.next().is_some() {
            return None;
        }

        if query_expr.spread.is_some() {
            return None;
        }

        let query = match &*query_expr.expr {
            swc_ecma_ast::Expr::Lit(lit) => lit.as_str()?.value.to_string(),
            swc_ecma_ast::Expr::Tpl(tpl) => tpl
                .quasis
                .iter()
                .map(|quasi| quasi.raw.to_string())
                .collect::<Vec<_>>()
                .join(""),
            _ => return None,
        };

        Some(Query {
            query: query.to_string(),
            lib: SupportedLib::NodePostgres,
        })
    }

    fn db_type_to_ts_type(&self, ty: &<Self::Db as sqlx::Database>::TypeInfo) -> TsType {
        match ty.name().to_lowercase().as_str() {
            "bool" => TS_BOOLEAN_TYPE,
            "line" | "polygon" | "path" | "lseg" | "jsonpath" | "tsrange" | "int4range"
            | "numrange" | "int8range" | "tstzrange" | "daterange" | "box" | "uuid" | "varbit"
            | "bit" | "numeric" | "text" | "varchar" | "bpchar" | "cidr" | "inet" | "int8"
            | "time" | "timetz" | "money" | "name" | "char" | "macaddr" | "macaddr8" => {
                TS_STRING_TYPE
            }
            "float4" | "float8" | "int2" | "int4" | "oid" => TS_NUMBER_TYPE,
            "timestamp" | "timestamptz" | "date" => ts_type_ref("Date"),
            "point" => ts_object_type([
                ("x".into(), TS_NUMBER_TYPE, false),
                ("y".into(), TS_NUMBER_TYPE, false),
            ]),
            "jsonb" | "json" => ts_type_ref("JsonValue"),
            "interval" => ts_object_type([
                ("milliseconds".into(), TS_NUMBER_TYPE, true),
                ("seconds".into(), TS_NUMBER_TYPE, true),
                ("minutes".into(), TS_NUMBER_TYPE, true),
                ("hours".into(), TS_NUMBER_TYPE, true),
                ("days".into(), TS_NUMBER_TYPE, true),
                ("months".into(), TS_NUMBER_TYPE, true),
                ("years".into(), TS_NUMBER_TYPE, true),
            ]),
            "bytea" => ts_type_ref("Buffer"),
            "circle" => ts_object_type([
                ("x".into(), TS_NUMBER_TYPE, false),
                ("y".into(), TS_NUMBER_TYPE, false),
                ("radius".into(), TS_NUMBER_TYPE, false),
            ]),
            _ => TS_UNKNOWN_TYPE,
        }
    }

    fn d_ts_prefix(&self) -> Vec<ModuleItem> {
        vec![
            ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                span: Default::default(),
                specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
                    span: Default::default(),
                    local: "QueryResult".into(),
                    imported: None,
                    is_type_only: false,
                })],
                src: Box::new(Str {
                    span: Default::default(),
                    value: "pg".into(),
                    raw: Some("'pg'".into()),
                }),
                type_only: true,
                with: None,
                phase: ImportPhase::Evaluation,
            })),
            ModuleItem::Stmt(Stmt::Decl(Decl::TsTypeAlias(Box::new(TsTypeAliasDecl {
                span: Span::default(),
                declare: false,
                id: "JsonValue".into(),
                type_params: None,
                type_ann: Box::new(TsType::TsUnionOrIntersectionType(
                    TsUnionOrIntersectionType::TsUnionType(TsUnionType {
                        span: Span::default(),
                        types: vec![
                            Box::new(TS_STRING_TYPE),
                            Box::new(TS_NUMBER_TYPE),
                            Box::new(TS_BOOLEAN_TYPE),
                            Box::new(TS_NULL_TYPE),
                            Box::new(TsType::TsMappedType(TsMappedType {
                                span: Span::default(),
                                readonly: None,
                                type_param: TsTypeParam {
                                    span: Span::default(),
                                    name: "Key".into(),
                                    is_in: false,
                                    is_out: false,
                                    is_const: false,
                                    constraint: Some(Box::new(TS_STRING_TYPE)),
                                    default: None,
                                },
                                name_type: None,
                                optional: Some(TruePlusMinus::True),
                                type_ann: Some(Box::new(ts_type_ref("JsonValue"))),
                            })),
                            Box::new(TsType::TsArrayType(TsArrayType {
                                span: Span::default(),
                                elem_type: Box::new(ts_type_ref("JsonValue")),
                            })),
                        ],
                    }),
                )),
            })))),
        ]
    }

    fn d_ts_suffix(&self) -> Vec<ModuleItem> {
        vec![ModuleItem::Stmt(Stmt::Decl(Decl::TsModule(Box::new(
            TsModuleDecl {
                span: Span::default(),
                declare: true,
                global: false,
                namespace: false,
                id: TsModuleName::Str(Str {
                    span: Span::default(),
                    value: "pg".into(),
                    raw: None,
                }),
                body: Some(TsNamespaceBody::TsModuleBlock(TsModuleBlock {
                    span: Span::default(),
                    body: vec![ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
                        span: Span::default(),
                        decl: Decl::TsInterface(Box::new(TsInterfaceDecl {
                            span: Span::default(),
                            id: "ClientBase".into(),
                            declare: false,
                            type_params: None,
                            extends: Vec::new(),
                            body: TsInterfaceBody {
                                span: Span::default(),
                                body: vec![TsTypeElement::TsMethodSignature(TsMethodSignature {
                                    span: Span::default(),
                                    key: "query".into(),
                                    computed: false,
                                    optional: false,
                                    params: vec![
                                        TsFnParam::Ident(BindingIdent {
                                            id: "q".into(),
                                            type_ann: Some(Box::new(TsTypeAnn {
                                                span: Span::default(),
                                                type_ann: Box::new(ts_type_ref("T")),
                                            })),
                                        }),
                                        TsFnParam::Ident(BindingIdent {
                                            id: "args".into(),
                                            type_ann: Some(Box::new(TsTypeAnn {
                                                span: Span::default(),
                                                type_ann: Box::new(TsType::TsConditionalType(
                                                    TsConditionalType {
                                                        span: Span::default(),
                                                        check_type: Box::new(ts_type_ref("T")),
                                                        extends_type: Box::new(
                                                            TsType::TsTypeOperator(
                                                                TsTypeOperator {
                                                                    span: Span::default(),
                                                                    op: TsTypeOperatorOp::KeyOf,
                                                                    type_ann: Box::new(
                                                                        ts_type_ref("Queries"),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        true_type: Box::new(
                                                            TsType::TsIndexedAccessType(
                                                                TsIndexedAccessType {
                                                                    span: Span::default(),
                                                                    readonly: false,
                                                                    obj_type: Box::new(
                                                                        TsType::TsIndexedAccessType(
                                                                            TsIndexedAccessType {
                                                                                span: Span::default(
                                                                                ),
                                                                                readonly: false,
                                                                                obj_type: Box::new(
                                                                                    ts_type_ref(
                                                                                        "Queries",
                                                                                    ),
                                                                                ),
                                                                                index_type:
                                                                                    Box::new(
                                                                                        ts_type_ref(
                                                                                            "T",
                                                                                        ),
                                                                                    ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                    index_type: Box::new(ts_lit_type("args")),
                                                                },
                                                            ),
                                                        ),
                                                        false_type: Box::new(TS_UNKNOWN_TYPE),
                                                    },
                                                )),
                                            })),
                                        }),
                                    ],
                                    type_ann: Some(Box::new(TsTypeAnn {
                                        span: Span::default(),
                                        type_ann: Box::new(TsType::TsTypeRef(TsTypeRef {
                                            span:Span::default(),
                                            type_name: TsEntityName::Ident("Promise".into()),
                                            type_params: Some(Box::new(TsTypeParamInstantiation {
                                                span: Span::default(),
                                                params: vec![Box::new(TsType::TsConditionalType (TsConditionalType {
                                                    span: Span::default(),
                                                    check_type: Box::new(ts_type_ref("T")),
                                                    extends_type: Box::new(TsType::TsTypeOperator(TsTypeOperator {
                                                        span: Span::default(),
                                                        op: TsTypeOperatorOp::KeyOf,
                                                        type_ann: Box::new(ts_type_ref("Queries"))
                                                    })),
                                                    true_type: Box::new(TsType::TsTypeRef(TsTypeRef {
                                                        span: Span::default(),
                                                        type_name: TsEntityName::Ident("QueryResult".into()),
                                                        type_params: Some(Box::new(TsTypeParamInstantiation {
                                                            span: Span::default(),
                                                            params: vec![
                                                                Box::new(TsType::TsIndexedAccessType(TsIndexedAccessType {
                                                                    span: Span::default(),
                                                                    readonly: false,
                                                                    obj_type: Box::new(TsType::TsIndexedAccessType(TsIndexedAccessType { span: Span::default(), readonly: false, obj_type: Box::new(ts_type_ref("Queries")), index_type: Box::new(ts_type_ref("T")) })),
                                                                    index_type: Box::new(ts_lit_type("returnType")) }))] })) })),
                                                    false_type: Box::new(TS_UNKNOWN_TYPE)
                                                }))]
                                            }))
                                        }))
                                    })),
                                    type_params: Some(Box::new(TsTypeParamDecl{span: Span::default(), params: vec![TsTypeParam { span: Span::default(), name: "T".into(), is_in: false, is_out: false, is_const: false, constraint: Some(Box::new(TS_STRING_TYPE)), default: None }]})),
                                })],
                            },
                        })),
                    }))],
                })),
            },
        ))))]
    }
}
