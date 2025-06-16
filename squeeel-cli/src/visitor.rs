use crate::SupportedLib;
use swc_ecma_ast::CallExpr;
use swc_ecma_ast::ModuleDecl;
use swc_ecma_ast::ObjectLit;
use swc_ecma_ast::Prop;
use swc_ecma_ast::PropOrSpread;
use swc_ecma_ast::{Decl, Expr, FnDecl, Module, ModuleItem, Stmt, VarDecl, VarDeclarator};

#[derive(Debug, Clone)]
pub struct Query {
    pub query: String,
    pub lib: SupportedLib,
}

pub struct AstVisitor<'a> {
    libs: &'a [SupportedLib],
    statements: Vec<Query>,
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

impl AstVisitor<'_> {
    fn visit_fn_decl(&mut self, fn_decl: &FnDecl) {
        let Some(body) = &fn_decl.function.body else {
            return;
        };

        for stmt in &body.stmts {
            self.visit_stmt(stmt);
        }
    }

    fn visit_call_expr(&mut self, call_expr: &CallExpr) {
        for lib in self.libs {
            if let Some(statement) = lib.parse_call_expr(call_expr) {
                self.statements.push(statement);
                return;
            }
        }
        match &call_expr.callee {
            swc_ecma_ast::Callee::Super(_) => {}
            swc_ecma_ast::Callee::Import(import) => {}
            swc_ecma_ast::Callee::Expr(expr) => self.visit_expr(expr),
        }
    }

    fn visit_prop(&mut self, prop: &Prop) {
        match prop {
            Prop::Shorthand(ident) => {}
            Prop::KeyValue(key_value_prop) => self.visit_expr(&key_value_prop.value),
            Prop::Assign(assign_prop) => todo!(),
            Prop::Getter(getter_prop) => todo!(),
            Prop::Setter(setter_prop) => todo!(),
            Prop::Method(method_prop) => {
                let Some(body) = &method_prop.function.body else {
                    return;
                };
                for stmt in &body.stmts {
                    self.visit_stmt(stmt);
                }
            }
        }
    }

    fn visit_prop_or_spread(&mut self, prop_or_spread: &PropOrSpread) {
        match prop_or_spread {
            swc_ecma_ast::PropOrSpread::Spread(spread_element) => {
                self.visit_expr(&spread_element.expr)
            }
            swc_ecma_ast::PropOrSpread::Prop(prop) => self.visit_prop(prop),
        }
    }

    fn visit_object_lit(&mut self, object_lit: &ObjectLit) {
        for prop in &object_lit.props {
            self.visit_prop_or_spread(prop);
        }
    }

    fn visit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::This(this_expr) => todo!(),
            Expr::Array(array_lit) => {}
            Expr::Object(object_lit) => self.visit_object_lit(object_lit),
            Expr::Fn(fn_expr) => todo!(),
            Expr::Unary(unary_expr) => self.visit_expr(&unary_expr.arg),
            Expr::Update(update_expr) => todo!(),
            Expr::Bin(bin_expr) => {
                self.visit_expr(&bin_expr.left);
                self.visit_expr(&bin_expr.right);
            }
            Expr::Assign(assign_expr) => self.visit_expr(&assign_expr.right),
            Expr::Member(member_expr) => self.visit_expr(&member_expr.obj),
            Expr::SuperProp(super_prop_expr) => todo!(),
            Expr::Cond(cond_expr) => {
                self.visit_expr(&cond_expr.test);
                self.visit_expr(&cond_expr.cons);
                self.visit_expr(&cond_expr.alt);
            }
            Expr::Call(call_expr) => self.visit_call_expr(call_expr),
            Expr::New(new_expr) => {}
            Expr::Seq(seq_expr) => todo!(),
            Expr::Ident(ident) => {}
            Expr::Lit(_) => {}
            Expr::Tpl(tpl) => {}
            Expr::TaggedTpl(tagged_tpl) => todo!(),
            Expr::Arrow(arrow_expr) => match &*arrow_expr.body {
                swc_ecma_ast::BlockStmtOrExpr::BlockStmt(block_stmt) => {
                    for stmt in &block_stmt.stmts {
                        self.visit_stmt(stmt);
                    }
                }
                swc_ecma_ast::BlockStmtOrExpr::Expr(expr) => self.visit_expr(expr),
            },
            Expr::Class(class_expr) => todo!(),
            Expr::Yield(yield_expr) => todo!(),
            Expr::MetaProp(meta_prop_expr) => todo!(),
            Expr::Await(await_expr) => self.visit_expr(&await_expr.arg),
            Expr::Paren(paren_expr) => self.visit_expr(&paren_expr.expr),
            Expr::JSXMember(jsxmember_expr) => todo!(),
            Expr::JSXNamespacedName(jsxnamespaced_name) => todo!(),
            Expr::JSXEmpty(jsxempty_expr) => todo!(),
            Expr::JSXElement(jsxelement) => todo!(),
            Expr::JSXFragment(jsxfragment) => todo!(),
            Expr::TsTypeAssertion(ts_type_assertion) => todo!(),
            Expr::TsConstAssertion(ts_const_assertion) => todo!(),
            Expr::TsNonNull(ts_non_null_expr) => self.visit_expr(&ts_non_null_expr.expr),
            Expr::TsAs(ts_as_expr) => self.visit_expr(&ts_as_expr.expr),
            Expr::TsInstantiation(ts_instantiation) => todo!(),
            Expr::TsSatisfies(ts_satisfies_expr) => todo!(),
            Expr::PrivateName(private_name) => todo!(),
            Expr::OptChain(opt_chain_expr) => match &*opt_chain_expr.base {
                swc_ecma_ast::OptChainBase::Member(member_expr) => {
                    self.visit_expr(&member_expr.obj)
                }
                swc_ecma_ast::OptChainBase::Call(opt_call) => self.visit_expr(&opt_call.callee),
            },
            Expr::Invalid(invalid) => todo!(),
        }
    }

    fn visit_var_declarator(&mut self, var_declarator: &VarDeclarator) {
        let Some(init) = &var_declarator.init else {
            return;
        };

        self.visit_expr(init)
    }

    fn visit_var_decl(&mut self, var_decl: &VarDecl) {
        for var_declarator in &var_decl.decls {
            self.visit_var_declarator(var_declarator);
        }
    }

    fn visit_decl(&mut self, decl: &Decl) {
        match decl {
            Decl::Class(class_decl) => todo!(),
            Decl::Fn(fn_decl) => self.visit_fn_decl(fn_decl),
            Decl::Var(var_decl) => self.visit_var_decl(var_decl),
            Decl::Using(using_decl) => todo!(),
            Decl::TsInterface(ts_interface_decl) => todo!(),
            Decl::TsTypeAlias(ts_type_alias_decl) => todo!(),
            Decl::TsEnum(ts_enum_decl) => todo!(),
            Decl::TsModule(ts_module_decl) => todo!(),
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            swc_ecma_ast::Stmt::Block(block_stmt) => {
                for stmt in &block_stmt.stmts {
                    self.visit_stmt(stmt);
                }
            }
            swc_ecma_ast::Stmt::Empty(empty_stmt) => todo!(),
            swc_ecma_ast::Stmt::Debugger(debugger_stmt) => todo!(),
            swc_ecma_ast::Stmt::With(_) => todo!(),
            swc_ecma_ast::Stmt::Return(return_stmt) => {
                let Some(expr) = &return_stmt.arg else {
                    return;
                };
                self.visit_expr(expr)
            }
            swc_ecma_ast::Stmt::Labeled(labeled_stmt) => todo!(),
            swc_ecma_ast::Stmt::Break(break_stmt) => todo!(),
            swc_ecma_ast::Stmt::Continue(continue_stmt) => todo!(),
            swc_ecma_ast::Stmt::If(if_stmt) => {
                self.visit_expr(&if_stmt.test);
                self.visit_stmt(&if_stmt.cons);
                if let Some(alt) = &if_stmt.alt {
                    self.visit_stmt(alt);
                }
            }
            swc_ecma_ast::Stmt::Switch(switch_stmt) => todo!(),
            swc_ecma_ast::Stmt::Throw(_) => {}
            swc_ecma_ast::Stmt::Try(try_stmt) => todo!(),
            swc_ecma_ast::Stmt::While(while_stmt) => self.visit_stmt(&while_stmt.body),
            swc_ecma_ast::Stmt::DoWhile(do_while_stmt) => todo!(),
            swc_ecma_ast::Stmt::For(for_stmt) => self.visit_stmt(&for_stmt.body),
            swc_ecma_ast::Stmt::ForIn(for_in_stmt) => todo!(),
            swc_ecma_ast::Stmt::ForOf(for_of_stmt) => {
                self.visit_expr(&for_of_stmt.right);
                self.visit_stmt(&for_of_stmt.body);
            }
            swc_ecma_ast::Stmt::Decl(decl) => self.visit_decl(decl),
            swc_ecma_ast::Stmt::Expr(expr_stmt) => self.visit_expr(&expr_stmt.expr),
        }
    }

    fn visit_module_decl(&mut self, module_decl: &ModuleDecl) {
        match module_decl {
            swc_ecma_ast::ModuleDecl::Import(import_decl) => {}
            swc_ecma_ast::ModuleDecl::ExportDecl(export_decl) => self.visit_decl(&export_decl.decl),
            swc_ecma_ast::ModuleDecl::ExportNamed(named_export) => todo!(),
            swc_ecma_ast::ModuleDecl::ExportDefaultDecl(export_default_decl) => todo!(),
            swc_ecma_ast::ModuleDecl::ExportDefaultExpr(export_default_expr) => {
                self.visit_expr(&export_default_expr.expr)
            }
            swc_ecma_ast::ModuleDecl::ExportAll(export_all) => todo!(),
            swc_ecma_ast::ModuleDecl::TsImportEquals(ts_import_equals_decl) => todo!(),
            swc_ecma_ast::ModuleDecl::TsExportAssignment(ts_export_assignment) => todo!(),
            swc_ecma_ast::ModuleDecl::TsNamespaceExport(ts_namespace_export_decl) => todo!(),
        }
    }

    fn visit_module_item(&mut self, module_item: &ModuleItem) {
        match module_item {
            ModuleItem::ModuleDecl(module_decl) => self.visit_module_decl(module_decl),
            ModuleItem::Stmt(stmt) => self.visit_stmt(stmt),
        }
    }

    pub fn visit(&mut self, module: &Module) {
        for module_item in &module.body {
            self.visit_module_item(module_item);
        }
    }
}
