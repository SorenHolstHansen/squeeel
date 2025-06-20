use crate::SupportedLib;
use swc_core::ecma::visit::Visit;
use swc_core::ecma::visit::VisitWith as _;
use swc_ecma_ast::CallExpr;

#[derive(Debug, Clone)]
pub struct Query {
    pub query: String,
    pub lib: SupportedLib,
}

pub struct AstVisitor<'a> {
    libs: &'a [SupportedLib],
    statements: Vec<Query>,
}

impl Visit for AstVisitor<'_> {
    fn visit_call_expr(&mut self, call_expr: &CallExpr) {
        for lib in self.libs {
            if let Some(statement) = lib.parse_call_expr(call_expr) {
                self.statements.push(statement);
                return;
            }
        }
        call_expr.visit_children_with(self);
    }
}

impl<'a> AstVisitor<'a> {
    pub fn new(libs: &'a [SupportedLib]) -> Self {
        Self {
            libs,
            statements: Vec::new(),
        }
    }

    pub fn statements(self) -> Vec<Query> {
        self.statements
    }
}
