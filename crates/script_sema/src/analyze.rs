//! Walk AST and check binding / scope / bool-condition rules (T-P1 / WP-L03).
//!
//! Spec:
//! - `PHASE-1-LANGUAGE-SPEC.md` §2.1 No Implicit New Binding by Assignment
//! - `PHASE-1-LANGUAGE-SPEC.md` §2.2 Block Scope Exists
//! - `PHASE-1-LANGUAGE-SPEC.md` §2.3 Conditions Must Be Boolean
//! - `PHASE-1-LANGUAGE-SPEC.md` §3.3 Unicode Normalization (NFC)
//! - declaration / assignment immutability; export visibility

use script_lex::Span;
use script_parse::{BinaryOp, Block, Decl, Expr, Item, Module, Stmt, UnaryOp};

use crate::binding::{nfc, Binding, BindingKind, ScopeStack};
use crate::error::SemaError;

/// Result of semantic analysis.
#[derive(Debug, Clone)]
pub struct SemaResult {
    /// Module-level user bindings introduced during analysis (NFC names).
    pub module_bindings: Vec<Binding>,
    pub errors: Vec<SemaError>,
}

impl SemaResult {
    #[must_use]
    pub fn ok(&self) -> bool {
        self.errors.is_empty()
    }
}

/// Conservative static type for SPEC-P1 §2.3 checks (not a full type system).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StaticTy {
    Bool,
    Int,
    Float,
    String,
    Nil,
    List,
    Map,
    /// Names, calls, or mixed forms — not rejected as non-Bool at frontend.
    Unknown,
}

struct Analyzer {
    scopes: ScopeStack,
    errors: Vec<SemaError>,
    module_snapshot: Vec<Binding>,
    loop_depth: u32,
    fn_depth: u32,
    /// When true, next `define` marks binding as exported.
    export_context: bool,
}

impl Analyzer {
    fn new() -> Self {
        Self {
            scopes: ScopeStack::new_module(),
            errors: Vec::new(),
            module_snapshot: Vec::new(),
            loop_depth: 0,
            fn_depth: 0,
            export_context: false,
        }
    }

    fn install_prelude(&mut self) {
        let _ = self.scopes.define(Binding::new(
            "print",
            BindingKind::Builtin,
            Span::empty(0),
            false,
        ));
    }

    fn err(&mut self, message: impl Into<String>, span: Span) {
        self.errors.push(SemaError::new(message, span));
    }

    fn define_clean(&mut self, name: String, kind: BindingKind, span: Span) {
        let binding = Binding::new(&name, kind, span, self.export_context);
        if let Err(msg) = self.scopes.define(binding.clone()) {
            self.err(msg, span);
            return;
        }
        if self.scopes.depth() == 1 && kind != BindingKind::Builtin {
            self.module_snapshot.push(binding);
        }
    }

    fn item(&mut self, item: &Item) {
        match item {
            Item::Decl(d) => self.decl(d),
            Item::Stmt(s) => self.stmt(s),
        }
    }

    fn decl(&mut self, decl: &Decl) {
        match decl {
            Decl::Let { name, value, span } => {
                self.expr(value);
                self.define_clean(name.clone(), BindingKind::Mutable, *span);
            }
            Decl::Const { name, value, span } => {
                self.expr(value);
                self.define_clean(name.clone(), BindingKind::Immutable, *span);
            }
            Decl::Function {
                name,
                params,
                body,
                span,
            } => {
                self.define_clean(name.clone(), BindingKind::Immutable, *span);
                self.scopes.push();
                for p in params {
                    self.define_clean(p.clone(), BindingKind::Mutable, *span);
                }
                self.fn_depth += 1;
                self.block(body);
                self.fn_depth -= 1;
                self.scopes.pop();
            }
            Decl::Import {
                module_path,
                alias,
                span,
            } => {
                let local = alias
                    .clone()
                    .unwrap_or_else(|| module_path.last().cloned().unwrap_or_default());
                if local.is_empty() {
                    self.err("import requires a module path", *span);
                } else {
                    self.define_clean(local, BindingKind::Immutable, *span);
                }
            }
            Decl::FromImport { items, span, .. } => {
                for item in items {
                    let local = item.alias.clone().unwrap_or_else(|| item.name.clone());
                    self.define_clean(local, BindingKind::Immutable, *span);
                }
            }
            Decl::Export { item, .. } => {
                let prev = self.export_context;
                self.export_context = true;
                self.decl(item);
                self.export_context = prev;
            }
        }
    }

    fn block(&mut self, block: &Block) {
        self.scopes.push();
        for s in &block.stmts {
            self.stmt(s);
        }
        self.scopes.pop();
    }

    fn stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr { expr, .. } => self.expr(expr),
            Stmt::Return { value, span } => {
                if self.fn_depth == 0 {
                    self.err("top-level `return` is invalid", *span);
                }
                if let Some(v) = value {
                    self.expr(v);
                }
            }
            Stmt::If {
                cond,
                then_block,
                elifs,
                else_block,
                ..
            } => {
                self.require_bool_condition(cond);
                self.block(then_block);
                for (c, b) in elifs {
                    self.require_bool_condition(c);
                    self.block(b);
                }
                if let Some(b) = else_block {
                    self.block(b);
                }
            }
            Stmt::While { cond, body, .. } => {
                self.require_bool_condition(cond);
                self.loop_depth += 1;
                self.block(body);
                self.loop_depth -= 1;
            }
            Stmt::For {
                name,
                iter,
                body,
                span,
            } => {
                self.expr(iter);
                self.loop_depth += 1;
                self.scopes.push();
                self.define_clean(name.clone(), BindingKind::Mutable, *span);
                for s in &body.stmts {
                    self.stmt(s);
                }
                self.scopes.pop();
                self.loop_depth -= 1;
            }
            Stmt::Break { span } => {
                if self.loop_depth == 0 {
                    self.err("`break` outside loop", *span);
                }
            }
            Stmt::Continue { span } => {
                if self.loop_depth == 0 {
                    self.err("`continue` outside loop", *span);
                }
            }
            Stmt::Assign { name, value, span } => {
                self.expr(value);
                self.check_mutable_assign(name, *span);
            }
            Stmt::AugAssign {
                name, value, span, ..
            } => {
                self.expr(value);
                self.check_mutable_assign(name, *span);
            }
            Stmt::Raise { value, .. } => self.expr(value),
            Stmt::Assert { cond, span } => {
                // Assert condition follows Bool rule (same as control conditions).
                let _ = span;
                self.require_bool_condition(cond);
            }
            Stmt::Decl(d) => self.decl(d),
        }
    }

    fn check_mutable_assign(&mut self, name: &str, span: Span) {
        let key = nfc(name);
        match self.scopes.resolve(&key) {
            None => self.err(
                format!(
                    "assignment to unbound name `{key}` (use `let`/`const`/`def` to introduce a binding)"
                ),
                span,
            ),
            Some(b) if b.kind != BindingKind::Mutable => self.err(
                format!("cannot assign to immutable binding `{key}`"),
                span,
            ),
            Some(_) => {}
        }
    }

    /// SPEC-P1 §2.3: control / logical conditions operate on Bool.
    fn require_bool_condition(&mut self, expr: &Expr) {
        self.expr_in_bool_context(expr);
        match self.static_ty(expr) {
            StaticTy::Bool | StaticTy::Unknown => {}
            other => {
                self.err(
                    format!(
                        "condition must be Bool (SPEC-P1 §2.3), found {other:?}"
                    ),
                    expr_span(expr),
                );
            }
        }
    }

    fn expr_in_bool_context(&mut self, expr: &Expr) {
        match expr {
            Expr::Unary {
                op: UnaryOp::Not,
                expr: inner,
                ..
            } => {
                self.require_bool_condition(inner);
            }
            Expr::Binary {
                op: BinaryOp::And | BinaryOp::Or,
                left,
                right,
                ..
            } => {
                self.require_bool_condition(left);
                self.require_bool_condition(right);
            }
            Expr::Binary {
                op:
                    BinaryOp::Eq
                    | BinaryOp::NotEq
                    | BinaryOp::Lt
                    | BinaryOp::LtEq
                    | BinaryOp::Gt
                    | BinaryOp::GtEq
                    | BinaryOp::Is
                    | BinaryOp::In,
                left,
                right,
                ..
            } => {
                self.expr(left);
                self.expr(right);
            }
            other => self.expr(other),
        }
    }

    fn expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Nil { .. }
            | Expr::Bool { .. }
            | Expr::Int { .. }
            | Expr::Float { .. }
            | Expr::String { .. } => {}
            Expr::Name { name, span } => {
                let key = nfc(name);
                if self.scopes.resolve(&key).is_none() {
                    self.err(format!("unresolved name `{key}`"), *span);
                }
            }
            Expr::Call { callee, args, .. } => {
                self.expr(callee);
                for a in args {
                    self.expr(a);
                }
            }
            Expr::Unary {
                op: UnaryOp::Not,
                expr: inner,
                span,
            } => {
                // `not` requires Bool operand (§2.3 / keyword operators).
                self.expr(inner);
                match self.static_ty(inner) {
                    StaticTy::Bool | StaticTy::Unknown => {}
                    other => self.err(
                        format!("`not` requires Bool operand, found {other:?}"),
                        *span,
                    ),
                }
            }
            Expr::Unary { expr: inner, .. } => self.expr(inner),
            Expr::Binary {
                op: BinaryOp::And | BinaryOp::Or,
                left,
                right,
                span,
            } => {
                self.expr(left);
                self.expr(right);
                for side in [left.as_ref(), right.as_ref()] {
                    match self.static_ty(side) {
                        StaticTy::Bool | StaticTy::Unknown => {}
                        other => self.err(
                            format!("logical operator requires Bool operands, found {other:?}"),
                            *span,
                        ),
                    }
                }
            }
            Expr::Binary { left, right, .. } => {
                self.expr(left);
                self.expr(right);
            }
            Expr::List { elements, .. } => {
                for e in elements {
                    self.expr(e);
                }
            }
            Expr::Map { entries, .. } => {
                for (k, v) in entries {
                    self.expr(k);
                    self.expr(v);
                }
            }
        }
    }

    fn static_ty(&self, expr: &Expr) -> StaticTy {
        match expr {
            Expr::Nil { .. } => StaticTy::Nil,
            Expr::Bool { .. } => StaticTy::Bool,
            Expr::Int { .. } => StaticTy::Int,
            Expr::Float { .. } => StaticTy::Float,
            Expr::String { .. } => StaticTy::String,
            Expr::List { .. } => StaticTy::List,
            Expr::Map { .. } => StaticTy::Map,
            Expr::Name { .. } | Expr::Call { .. } => StaticTy::Unknown,
            Expr::Unary {
                op: UnaryOp::Not, ..
            } => StaticTy::Bool,
            Expr::Unary {
                op: UnaryOp::Neg, ..
            } => StaticTy::Int,
            Expr::Binary { op, .. } => match op {
                BinaryOp::And
                | BinaryOp::Or
                | BinaryOp::Eq
                | BinaryOp::NotEq
                | BinaryOp::Lt
                | BinaryOp::LtEq
                | BinaryOp::Gt
                | BinaryOp::GtEq
                | BinaryOp::Is
                | BinaryOp::In => StaticTy::Bool,
                BinaryOp::Add
                | BinaryOp::Sub
                | BinaryOp::Mul
                | BinaryOp::Div
                | BinaryOp::Rem => StaticTy::Int,
            },
        }
    }
}

fn expr_span(expr: &Expr) -> Span {
    match expr {
        Expr::Nil { span }
        | Expr::Bool { span, .. }
        | Expr::Int { span, .. }
        | Expr::Float { span, .. }
        | Expr::String { span, .. }
        | Expr::Name { span, .. }
        | Expr::Call { span, .. }
        | Expr::Unary { span, .. }
        | Expr::Binary { span, .. }
        | Expr::List { span, .. }
        | Expr::Map { span, .. } => *span,
    }
}

/// Analyze a parsed module for binding/scope/bool-condition errors.
pub fn analyze_module(module: &Module) -> SemaResult {
    let mut ctx = Analyzer::new();
    ctx.install_prelude();
    for item in &module.items {
        ctx.item(item);
    }
    SemaResult {
        module_bindings: ctx.module_snapshot,
        errors: ctx.errors,
    }
}

/// Parse + analyze source in one step.
pub fn check_source(source: &str) -> Result<SemaResult, crate::CheckError> {
    let module = script_parse::parse_module(source).map_err(crate::CheckError::Parse)?;
    Ok(analyze_module(&module))
}

#[cfg(test)]
mod tests {
    use super::*;
    use script_parse::parse_module;

    fn check(src: &str) -> SemaResult {
        let m = parse_module(src).expect("parse");
        analyze_module(&m)
    }

    #[test]
    fn let_then_assign_ok() {
        let r = check("let x = 1\nx = 2\n");
        assert!(r.ok(), "{:?}", r.errors);
    }

    #[test]
    fn assign_without_let_fails() {
        let r = check("x = 1\n");
        assert!(!r.ok());
        assert!(r.errors[0].message.contains("unbound"));
    }

    #[test]
    fn const_reassign_fails() {
        let r = check("const x = 1\nx = 2\n");
        assert!(!r.ok());
        assert!(r.errors.iter().any(|e| e.message.contains("immutable")));
    }

    #[test]
    fn def_name_immutable() {
        let r = check("def f():\n    return 1\nf = 2\n");
        assert!(!r.ok());
        assert!(r.errors.iter().any(|e| e.message.contains("immutable")));
    }

    #[test]
    fn block_scope_does_not_leak() {
        let src = "if true:\n    let x = 1\nx = 2\n";
        let r = check(src);
        assert!(!r.ok());
        assert!(r.errors.iter().any(|e| e.message.contains("unbound")));
    }

    #[test]
    fn duplicate_in_same_scope() {
        let r = check("let x = 1\nlet x = 2\n");
        assert!(!r.ok());
        assert!(r.errors.iter().any(|e| e.message.contains("duplicate")));
    }

    #[test]
    fn function_params_and_body() {
        let r = check("def add(a, b):\n    return a + b\n");
        assert!(r.ok(), "{:?}", r.errors);
    }

    #[test]
    fn unresolved_name_in_expr() {
        let r = check("let x = y\n");
        assert!(!r.ok());
        assert!(r.errors.iter().any(|e| e.message.contains("unresolved")));
    }

    #[test]
    fn fib_with_print_prelude() {
        let src = r#"
def fib(n):
    if n < 2:
        return n
    return fib(n - 1) + fib(n - 2)

print(fib(10))
"#;
        let r = check(src);
        assert!(r.ok(), "{:?}", r.errors);
        assert!(r.module_bindings.iter().any(|b| b.name == "fib"));
    }

    #[test]
    fn nested_let_shadow_ok() {
        let src = "let x = 1\nif true:\n    let x = 2\n";
        let r = check(src);
        assert!(r.ok(), "{:?}", r.errors);
    }

    #[test]
    fn break_outside_loop_fails() {
        let r = check("break\n");
        assert!(!r.ok());
        assert!(r.errors.iter().any(|e| e.message.contains("break")));
    }

    #[test]
    fn for_loop_binds_iterator_var() {
        let src = "let xs = [1]\nfor x in xs:\n    let y = x\n";
        let r = check(src);
        assert!(r.ok(), "{:?}", r.errors);
    }

    #[test]
    fn top_level_return_invalid() {
        let r = check("return 1\n");
        assert!(!r.ok());
        assert!(r.errors.iter().any(|e| e.message.contains("return")));
    }

    #[test]
    fn if_condition_rejects_int_literal() {
        // SPEC-P1 §2.3
        let r = check("if 1:\n    let x = 1\n");
        assert!(!r.ok());
        assert!(r.errors.iter().any(|e| e.message.contains("Bool")));
    }

    #[test]
    fn while_condition_rejects_string() {
        let r = check("while \"x\":\n    break\n");
        assert!(!r.ok());
        assert!(r.errors.iter().any(|e| e.message.contains("Bool")));
    }

    #[test]
    fn comparison_condition_ok() {
        let r = check("let n = 1\nif n < 2:\n    let x = 1\n");
        assert!(r.ok(), "{:?}", r.errors);
    }

    #[test]
    fn logical_and_requires_bool_operands() {
        let r = check("let x = 1 and 2\n");
        assert!(!r.ok());
        assert!(r.errors.iter().any(|e| e.message.contains("Bool")));
    }

    #[test]
    fn logical_and_bool_ok() {
        let r = check("let x = true and false\n");
        assert!(r.ok(), "{:?}", r.errors);
    }

    #[test]
    fn nfc_same_scope_clash() {
        // e + combining acute vs precomposed é — same NFC
        let src = "let e\u{0301} = 1\nlet \u{00e9} = 2\n";
        let r = check(src);
        assert!(!r.ok());
        assert!(r.errors.iter().any(|e| e.message.contains("duplicate")));
    }

    #[test]
    fn export_marks_binding() {
        let r = check("export let x = 1\n");
        assert!(r.ok(), "{:?}", r.errors);
        let b = r
            .module_bindings
            .iter()
            .find(|b| b.name == "x")
            .expect("x");
        assert!(b.exported);
    }

    #[test]
    fn non_export_not_marked() {
        let r = check("let x = 1\n");
        assert!(r.ok(), "{:?}", r.errors);
        let b = r.module_bindings.iter().find(|b| b.name == "x").unwrap();
        assert!(!b.exported);
    }

    #[test]
    fn assert_rejects_non_bool() {
        let r = check("assert 1\n");
        assert!(!r.ok());
        assert!(r.errors.iter().any(|e| e.message.contains("Bool")));
    }
}
