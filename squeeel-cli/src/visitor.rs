use crate::SupportedLib;
use swc_ecma_ast::AssignTarget;
use swc_ecma_ast::AssignTargetPat;
use swc_ecma_ast::CallExpr;
use swc_ecma_ast::ClassDecl;
use swc_ecma_ast::ClassMember;
use swc_ecma_ast::DefaultDecl;
use swc_ecma_ast::MemberExpr;
use swc_ecma_ast::ModuleDecl;
use swc_ecma_ast::ObjectLit;
use swc_ecma_ast::ObjectPatProp;
use swc_ecma_ast::Pat;
use swc_ecma_ast::Prop;
use swc_ecma_ast::PropOrSpread;
use swc_ecma_ast::SimpleAssignTarget;
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
            swc_ecma_ast::Callee::Import(_import) => {}
            swc_ecma_ast::Callee::Expr(expr) => self.visit_expr(expr),
        }
    }

    fn visit_prop(&mut self, prop: &Prop) {
        match prop {
            Prop::Shorthand(_ident) => {}
            Prop::KeyValue(key_value_prop) => self.visit_expr(&key_value_prop.value),
            Prop::Assign(assign_prop) => self.visit_expr(&assign_prop.value),
            Prop::Getter(getter_prop) => {
                if let Some(body) = &getter_prop.body {
                    for stmt in &body.stmts {
                        self.visit_stmt(stmt);
                    }
                }
            }
            Prop::Setter(setter_prop) => {
                if let Some(body) = &setter_prop.body {
                    for stmt in &body.stmts {
                        self.visit_stmt(stmt);
                    }
                }
            }
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

    fn visit_member_expr(&mut self, member_expr: &MemberExpr) {
        self.visit_expr(&member_expr.obj);
        if let Some(computed_prop_name) = &member_expr.prop.as_computed() {
            self.visit_expr(&computed_prop_name.expr);
        }
    }

    fn visit_simple_assign_target(&mut self, simple_assign_target: &SimpleAssignTarget) {
        match simple_assign_target {
            swc_ecma_ast::SimpleAssignTarget::Ident(_binding_ident) => {}
            swc_ecma_ast::SimpleAssignTarget::Member(member_expr) => {
                self.visit_member_expr(member_expr);
            }
            swc_ecma_ast::SimpleAssignTarget::SuperProp(super_prop_expr) => {
                if let Some(computed) = super_prop_expr.prop.as_computed() {
                    self.visit_expr(&computed.expr);
                }
            }
            swc_ecma_ast::SimpleAssignTarget::Paren(paren_expr) => {
                self.visit_expr(&paren_expr.expr)
            }
            swc_ecma_ast::SimpleAssignTarget::OptChain(opt_chain_expr) => {
                match &*opt_chain_expr.base {
                    swc_ecma_ast::OptChainBase::Member(member_expr) => {
                        self.visit_member_expr(member_expr)
                    }
                    swc_ecma_ast::OptChainBase::Call(opt_call) => {
                        self.visit_expr(&opt_call.callee);
                        for arg in &opt_call.args {
                            self.visit_expr(&arg.expr);
                        }
                    }
                }
            }
            swc_ecma_ast::SimpleAssignTarget::TsAs(ts_as_expr) => self.visit_expr(&ts_as_expr.expr),
            swc_ecma_ast::SimpleAssignTarget::TsSatisfies(ts_satisfies_expr) => {
                self.visit_expr(&ts_satisfies_expr.expr)
            }
            swc_ecma_ast::SimpleAssignTarget::TsNonNull(ts_non_null_expr) => {
                self.visit_expr(&ts_non_null_expr.expr)
            }
            swc_ecma_ast::SimpleAssignTarget::TsTypeAssertion(ts_type_assertion) => {
                self.visit_expr(&ts_type_assertion.expr)
            }
            swc_ecma_ast::SimpleAssignTarget::TsInstantiation(ts_instantiation) => {
                self.visit_expr(&ts_instantiation.expr)
            }
            swc_ecma_ast::SimpleAssignTarget::Invalid(_invalid) => {}
        }
    }

    fn visit_object_path_prop(&mut self, object_pat_prop: &ObjectPatProp) {
        match object_pat_prop {
            ObjectPatProp::KeyValue(key_value_pat_prop) => {
                if let Some(computed) = key_value_pat_prop.key.as_computed() {
                    self.visit_expr(&computed.expr);
                }
                self.visit_pat(&key_value_pat_prop.value);
            }
            ObjectPatProp::Assign(assign_pat_prop) => {
                if let Some(value) = &assign_pat_prop.value {
                    self.visit_expr(value);
                }
            }
            ObjectPatProp::Rest(rest_pat) => self.visit_pat(&rest_pat.arg),
        }
    }

    fn visit_pat(&mut self, pat: &Pat) {
        match pat {
            Pat::Ident(_binding_ident) => {}
            Pat::Array(array_pat) => {
                for elem in array_pat.elems.iter().flatten() {
                    self.visit_pat(elem);
                }
            }
            Pat::Rest(rest_pat) => self.visit_pat(&rest_pat.arg),
            Pat::Object(object_pat) => {
                for prop in &object_pat.props {
                    self.visit_object_path_prop(prop);
                }
            }
            Pat::Assign(assign_pat) => {
                self.visit_pat(&assign_pat.left);
                self.visit_expr(&assign_pat.right);
            }
            Pat::Invalid(_invalid) => {}
            Pat::Expr(expr) => self.visit_expr(expr),
        }
    }

    fn visit_assign_target_pat(&mut self, assign_target_pat: &AssignTargetPat) {
        match assign_target_pat {
            AssignTargetPat::Array(array_pat) => {
                for elem in array_pat.elems.iter().flatten() {
                    self.visit_pat(elem);
                }
            }
            AssignTargetPat::Object(object_pat) => {
                for prop in &object_pat.props {
                    self.visit_object_path_prop(prop)
                }
            }
            AssignTargetPat::Invalid(_invalid) => todo!(),
        }
    }

    fn visit_assign_target(&mut self, assign_target: &AssignTarget) {
        match assign_target {
            AssignTarget::Simple(simple_assign_target) => {
                self.visit_simple_assign_target(simple_assign_target)
            }
            AssignTarget::Pat(assign_target_pat) => self.visit_assign_target_pat(assign_target_pat),
        }
    }

    fn visit_class_member(&mut self, class_member: &ClassMember) {
        match class_member {
            swc_ecma_ast::ClassMember::Constructor(constructor) => {
                if let Some(body) = &constructor.body {
                    for stmt in &body.stmts {
                        self.visit_stmt(stmt);
                    }
                }
            }
            swc_ecma_ast::ClassMember::Method(class_method) => {
                if let Some(body) = &class_method.function.body {
                    for stmt in &body.stmts {
                        self.visit_stmt(stmt);
                    }
                }
            }
            swc_ecma_ast::ClassMember::PrivateMethod(private_method) => {
                if let Some(body) = &private_method.function.body {
                    for stmt in &body.stmts {
                        self.visit_stmt(stmt);
                    }
                }
            }
            swc_ecma_ast::ClassMember::ClassProp(class_prop) => {
                if let Some(value) = &class_prop.value {
                    self.visit_expr(value);
                }
            }
            swc_ecma_ast::ClassMember::PrivateProp(private_prop) => {
                if let Some(value) = &private_prop.value {
                    self.visit_expr(value);
                }
            }
            swc_ecma_ast::ClassMember::TsIndexSignature(_ts_index_signature) => {}
            swc_ecma_ast::ClassMember::Empty(_empty_stmt) => {}
            swc_ecma_ast::ClassMember::StaticBlock(static_block) => {
                for stmt in &static_block.body.stmts {
                    self.visit_stmt(stmt);
                }
            }
            swc_ecma_ast::ClassMember::AutoAccessor(auto_accessor) => {
                if let Some(value) = &auto_accessor.value {
                    self.visit_expr(value);
                }
            }
        }
    }

    fn visit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::This(_this_expr) => {}
            Expr::Array(array_lit) => {
                for elem in array_lit.elems.iter().flatten() {
                    self.visit_expr(&elem.expr);
                }
            }
            Expr::Object(object_lit) => self.visit_object_lit(object_lit),
            Expr::Fn(fn_expr) => {
                if let Some(body) = &fn_expr.function.body {
                    for stmt in &body.stmts {
                        self.visit_stmt(stmt);
                    }
                }
            }
            Expr::Unary(unary_expr) => self.visit_expr(&unary_expr.arg),
            Expr::Update(update_expr) => self.visit_expr(&update_expr.arg),
            Expr::Bin(bin_expr) => {
                self.visit_expr(&bin_expr.left);
                self.visit_expr(&bin_expr.right);
            }
            Expr::Assign(assign_expr) => {
                self.visit_expr(&assign_expr.right);
                self.visit_assign_target(&assign_expr.left);
            }
            Expr::Member(member_expr) => {
                self.visit_member_expr(member_expr);
            }
            Expr::SuperProp(super_prop_expr) => {
                if let Some(computed) = super_prop_expr.prop.as_computed() {
                    self.visit_expr(&computed.expr);
                }
            }
            Expr::Cond(cond_expr) => {
                self.visit_expr(&cond_expr.test);
                self.visit_expr(&cond_expr.cons);
                self.visit_expr(&cond_expr.alt);
            }
            Expr::Call(call_expr) => self.visit_call_expr(call_expr),
            Expr::New(new_expr) => {
                self.visit_expr(&new_expr.callee);
                if let Some(args) = &new_expr.args {
                    for arg in args {
                        self.visit_expr(&arg.expr);
                    }
                }
            }
            Expr::Seq(seq_expr) => {
                for expr in &seq_expr.exprs {
                    self.visit_expr(expr);
                }
            }
            Expr::Ident(_ident) => {}
            Expr::Lit(_) => {}
            Expr::Tpl(tpl) => {
                for expr in &tpl.exprs {
                    self.visit_expr(expr);
                }
            }
            Expr::TaggedTpl(tagged_tpl) => {
                self.visit_expr(&tagged_tpl.tag);
                for expr in &tagged_tpl.tpl.exprs {
                    self.visit_expr(expr);
                }
            }
            Expr::Arrow(arrow_expr) => match &*arrow_expr.body {
                swc_ecma_ast::BlockStmtOrExpr::BlockStmt(block_stmt) => {
                    for stmt in &block_stmt.stmts {
                        self.visit_stmt(stmt);
                    }
                }
                swc_ecma_ast::BlockStmtOrExpr::Expr(expr) => self.visit_expr(expr),
            },
            Expr::Class(class_expr) => {
                for class_member in &class_expr.class.body {
                    self.visit_class_member(class_member);
                }
            }
            Expr::Yield(yield_expr) => {
                if let Some(arg) = &yield_expr.arg {
                    self.visit_expr(arg);
                }
            }
            Expr::MetaProp(_meta_prop_expr) => {}
            Expr::Await(await_expr) => self.visit_expr(&await_expr.arg),
            Expr::Paren(paren_expr) => self.visit_expr(&paren_expr.expr),
            Expr::JSXMember(_jsxmember_expr) => {}
            Expr::JSXNamespacedName(_jsxnamespaced_name) => {}
            Expr::JSXEmpty(_jsxempty_expr) => {}
            Expr::JSXElement(_jsxelement) => {}
            Expr::JSXFragment(_jsxfragment) => {}
            Expr::TsTypeAssertion(ts_type_assertion) => self.visit_expr(&ts_type_assertion.expr),
            Expr::TsConstAssertion(ts_const_assertion) => self.visit_expr(&ts_const_assertion.expr),
            Expr::TsNonNull(ts_non_null_expr) => self.visit_expr(&ts_non_null_expr.expr),
            Expr::TsAs(ts_as_expr) => self.visit_expr(&ts_as_expr.expr),
            Expr::TsInstantiation(ts_instantiation) => self.visit_expr(&ts_instantiation.expr),
            Expr::TsSatisfies(ts_satisfies_expr) => self.visit_expr(&ts_satisfies_expr.expr),
            Expr::PrivateName(_private_name) => {}
            Expr::OptChain(opt_chain_expr) => match &*opt_chain_expr.base {
                swc_ecma_ast::OptChainBase::Member(member_expr) => {
                    self.visit_expr(&member_expr.obj)
                }
                swc_ecma_ast::OptChainBase::Call(opt_call) => self.visit_expr(&opt_call.callee),
            },
            Expr::Invalid(_invalid) => {}
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

    fn visit_class_decl(&mut self, class_decl: &ClassDecl) {
        for class_member in &class_decl.class.body {
            self.visit_class_member(class_member);
        }
    }

    fn visit_decl(&mut self, decl: &Decl) {
        match decl {
            Decl::Class(class_decl) => self.visit_class_decl(class_decl),
            Decl::Fn(fn_decl) => self.visit_fn_decl(fn_decl),
            Decl::Var(var_decl) => self.visit_var_decl(var_decl),
            Decl::Using(_using_decl) => {}
            Decl::TsInterface(_ts_interface_decl) => {}
            Decl::TsTypeAlias(_ts_type_alias_decl) => {}
            Decl::TsEnum(_ts_enum_decl) => {}
            Decl::TsModule(_ts_module_decl) => {}
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            swc_ecma_ast::Stmt::Block(block_stmt) => {
                for stmt in &block_stmt.stmts {
                    self.visit_stmt(stmt);
                }
            }
            swc_ecma_ast::Stmt::Empty(_empty_stmt) => {}
            swc_ecma_ast::Stmt::Debugger(_debugger_stmt) => {}
            swc_ecma_ast::Stmt::With(with) => {
                self.visit_expr(&with.obj);
                self.visit_stmt(&with.body);
            }
            swc_ecma_ast::Stmt::Return(return_stmt) => {
                if let Some(expr) = &return_stmt.arg {
                    self.visit_expr(expr)
                };
            }
            swc_ecma_ast::Stmt::Labeled(labeled_stmt) => self.visit_stmt(&labeled_stmt.body),
            swc_ecma_ast::Stmt::Break(_break_stmt) => {}
            swc_ecma_ast::Stmt::Continue(_continue_stmt) => {}
            swc_ecma_ast::Stmt::If(if_stmt) => {
                self.visit_expr(&if_stmt.test);
                self.visit_stmt(&if_stmt.cons);
                if let Some(alt) = &if_stmt.alt {
                    self.visit_stmt(alt);
                }
            }
            swc_ecma_ast::Stmt::Switch(switch_stmt) => {
                self.visit_expr(&switch_stmt.discriminant);
                for case in &switch_stmt.cases {
                    for con in &case.cons {
                        self.visit_stmt(con);
                    }
                }
            }
            swc_ecma_ast::Stmt::Throw(_) => {}
            swc_ecma_ast::Stmt::Try(try_stmt) => {
                for stmt in &try_stmt.block.stmts {
                    self.visit_stmt(stmt);
                }
                if let Some(handler) = &try_stmt.handler {
                    for stmt in &handler.body.stmts {
                        self.visit_stmt(stmt);
                    }
                }
                if let Some(finalizer) = &try_stmt.finalizer {
                    for stmt in &finalizer.stmts {
                        self.visit_stmt(stmt);
                    }
                }
            }
            swc_ecma_ast::Stmt::While(while_stmt) => {
                self.visit_expr(&while_stmt.test);
                self.visit_stmt(&while_stmt.body);
            }
            swc_ecma_ast::Stmt::DoWhile(do_while_stmt) => {
                self.visit_expr(&do_while_stmt.test);
                self.visit_stmt(&do_while_stmt.body);
            }
            swc_ecma_ast::Stmt::For(for_stmt) => self.visit_stmt(&for_stmt.body),
            swc_ecma_ast::Stmt::ForIn(for_in_stmt) => {
                self.visit_expr(&for_in_stmt.right);
                self.visit_stmt(&for_in_stmt.body);
            }
            swc_ecma_ast::Stmt::ForOf(for_of_stmt) => {
                self.visit_expr(&for_of_stmt.right);
                self.visit_stmt(&for_of_stmt.body);
            }
            swc_ecma_ast::Stmt::Decl(decl) => self.visit_decl(decl),
            swc_ecma_ast::Stmt::Expr(expr_stmt) => self.visit_expr(&expr_stmt.expr),
        }
    }

    fn visit_default_decl(&mut self, default_decl: &DefaultDecl) {
        match default_decl {
            DefaultDecl::Class(class_expr) => {
                for class_member in &class_expr.class.body {
                    self.visit_class_member(class_member);
                }
            }
            DefaultDecl::Fn(fn_expr) => {
                if let Some(body) = &fn_expr.function.body {
                    for stmt in &body.stmts {
                        self.visit_stmt(stmt);
                    }
                }
            }
            DefaultDecl::TsInterfaceDecl(_ts_interface_decl) => {}
        }
    }

    fn visit_module_decl(&mut self, module_decl: &ModuleDecl) {
        match module_decl {
            swc_ecma_ast::ModuleDecl::Import(_import_decl) => {}
            swc_ecma_ast::ModuleDecl::ExportDecl(export_decl) => self.visit_decl(&export_decl.decl),
            swc_ecma_ast::ModuleDecl::ExportNamed(_named_export) => {}
            swc_ecma_ast::ModuleDecl::ExportDefaultDecl(export_default_decl) => {
                self.visit_default_decl(&export_default_decl.decl);
            }
            swc_ecma_ast::ModuleDecl::ExportDefaultExpr(export_default_expr) => {
                self.visit_expr(&export_default_expr.expr)
            }
            swc_ecma_ast::ModuleDecl::ExportAll(_export_all) => {}
            swc_ecma_ast::ModuleDecl::TsImportEquals(_ts_import_equals_decl) => {}
            swc_ecma_ast::ModuleDecl::TsExportAssignment(ts_export_assignment) => {
                self.visit_expr(&ts_export_assignment.expr)
            }
            swc_ecma_ast::ModuleDecl::TsNamespaceExport(_ts_namespace_export_decl) => {}
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
