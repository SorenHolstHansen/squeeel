use crate::SupportedLib;
use std::path::Path;
use swc_core::ecma::visit::Visit;
use swc_core::ecma::visit::VisitWith as _;
use swc_ecma_ast::CallExpr;
use swc_ecma_ast::Module;

#[derive(Debug, Clone)]
pub struct Query {
    pub query: String,
    pub lib: SupportedLib,
}

struct AstVisitor<'a> {
    path: &'a Path,
    libs: &'a [SupportedLib],
    statements: Vec<Query>,
    errors: Vec<String>,
}

impl Visit for AstVisitor<'_> {
    fn visit_call_expr(&mut self, call_expr: &CallExpr) {
        let mut libs_that_detected_a_query = Vec::new();
        for lib in self.libs {
            if let Some(statement) = lib.parse_call_expr(call_expr) {
                self.statements.push(statement);
                libs_that_detected_a_query.push(lib);
            }
        }
        if libs_that_detected_a_query.is_empty() {
            call_expr.visit_children_with(self);
        }
        if libs_that_detected_a_query.len() > 1 {
            self.errors.push(format!(
                "Multiple libs ({}) detected the same query in {}:{:?}.",
                libs_that_detected_a_query
                    .iter()
                    .map(|l| l.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
                self.path.to_string_lossy(),
                call_expr.span
            ));
        }
    }
}

impl<'a> AstVisitor<'a> {
    pub fn new(libs: &'a [SupportedLib], path: &'a Path) -> Self {
        Self {
            path,
            libs,
            statements: Vec::new(),
            errors: Vec::new(),
        }
    }
}

pub fn visit_ast(
    supported_libs: &[SupportedLib],
    module: &Module,
    path: &Path,
) -> Result<Vec<Query>, Vec<String>> {
    let mut ast_visitor = AstVisitor::new(supported_libs, path);
    ast_visitor.visit_module(module);
    if !ast_visitor.errors.is_empty() {
        return Err(ast_visitor.errors);
    }
    Ok(ast_visitor.statements)
}
