//! Walk AST and check binding / scope rules (bootstrap).
//!
//! Spec:
//! - `PHASE-1-LANGUAGE-SPEC.md` §2.1 No Implicit New Binding by Assignment
//! - `PHASE-1-LANGUAGE-SPEC.md` §2.2 Block Scope Exists
//! - let / const / def introduction; assignment requires mutable binding

use script_lex::Span;
use script_parse::{Block, Decl, Expr, Item, Module, Stmt};

use crate::binding::{Binding, BindingKind, ScopeStack};
use crate::error::SemaError;

/// Result of semantic analysis.
#[derive(Debug, Clone)]
pub struct SemaResult {
    /// Module-level user bindings introduced during analysis.
    pub module_bindings: Vec<Binding>,
    pub errors: Vec<SemaError>,
}

impl SemaResult {
    #[must_use]
    pub fn ok(&self) -> bool {
        self.errors.is_empty()
    }
}

struct Analyzer {
    scopes: ScopeStack,
    errors: Vec<SemaError>,
    module_snapshot: Vec<Binding>,
    /// Nesting depth of `while` / `for` for break/continue validity.
    loop_depth: u32,
    /// Nesting depth of function bodies for top-level `return` rejection.
    fn_depth: u32,
}

impl Analyzer {
    fn new() -> Self {
        Self {
            scopes: ScopeStack::new_module(),
            errors: Vec::new(),
            module_snapshot: Vec::new(),
            loop_depth: 0,
            fn_depth: 0,
        }
    }

    fn install_prelude(&mut self) {
        // Minimal host-facing names used by bootstrap samples (not full stdlib).
        let _ = self.scopes.define(Binding {
            name: "print".into(),
            kind: BindingKind::Builtin,
            decl_span: Span::empty(0),
        });
    }

    fn err(&mut self, message: impl Into<String>, span: Span) {
        self.errors.push(SemaError::new(message, span));
    }

    fn define(&mut self, name: String, kind: BindingKind, span: Span) {
        let binding = Binding {
            name: name.clone(),
            kind,
            decl_span: span,
        };
        if let Err(msg) = self.scopes.define(binding.clone()) {
            self.err(msg, span);
            return;
        }
        if self.scopes.depth() == 1 {
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
                self.define(name.clone(), BindingKind::Mutable, *span);
            }
            Decl::Const { name, value, span } => {
                self.expr(value);
                self.define(name.clone(), BindingKind::Immutable, *span);
            }
            Decl::Function {
                name,
                params,
                body,
                span,
            } => {
                self.define(name.clone(), BindingKind::Immutable, *span);
                self.scopes.push();
                for p in params {
                    self.define(p.clone(), BindingKind::Mutable, *span);
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
                    // Import introduces an immutable binding (module object / namespace bootstrap).
                    self.define(local, BindingKind::Immutable, *span);
                }
            }
            Decl::FromImport { items, span, .. } => {
                for item in items {
                    let local = item.alias.clone().unwrap_or_else(|| item.name.clone());
                    self.define(local, BindingKind::Immutable, *span);
                }
            }
            Decl::Export { item, .. } => self.decl(item),
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
                self.expr(cond);
                self.block(then_block);
                for (c, b) in elifs {
                    self.expr(c);
                    self.block(b);
                }
                if let Some(b) = else_block {
                    self.block(b);
                }
            }
            Stmt::While { cond, body, .. } => {
                self.expr(cond);
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
                self.define(name.clone(), BindingKind::Mutable, *span);
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
            Stmt::Assert { cond, .. } => self.expr(cond),
            Stmt::Decl(d) => self.decl(d),
        }
    }

    fn check_mutable_assign(&mut self, name: &str, span: Span) {
        match self.scopes.resolve(name) {
            None => self.err(
                format!(
                    "assignment to unbound name `{name}` (use `let`/`const`/`def` to introduce a binding)"
                ),
                span,
            ),
            Some(b) if b.kind != BindingKind::Mutable => self.err(
                format!("cannot assign to immutable binding `{name}`"),
                span,
            ),
            Some(_) => {}
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
                if self.scopes.resolve(name).is_none() {
                    self.err(format!("unresolved name `{name}`"), *span);
                }
            }
            Expr::Call { callee, args, .. } => {
                self.expr(callee);
                for a in args {
                    self.expr(a);
                }
            }
            Expr::Unary { expr, .. } => self.expr(expr),
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
}

/// Analyze a parsed module for binding/scope errors.
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
}
