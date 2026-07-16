//! Binding table and scope stack.
//!
//! Spec: `PHASE-1-LANGUAGE-SPEC.md` §2.1–2.2, declaration immutability rules

use script_lex::Span;

/// How a name may be used after introduction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingKind {
    /// `let` — mutable binding.
    Mutable,
    /// `const` or `def` — immutable binding (cannot reassign the name).
    Immutable,
    /// Prelude / host-injected name (treated as immutable for assignment).
    Builtin,
}

/// One name binding in a scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binding {
    pub name: String,
    pub kind: BindingKind,
    pub decl_span: Span,
}

/// Lexical scope: map of NFC names → binding.
#[derive(Debug, Clone, Default)]
pub struct Scope {
    pub bindings: Vec<Binding>,
}

impl Scope {
    #[must_use]
    pub fn new() -> Self {
        Self {
            bindings: Vec::new(),
        }
    }

    pub fn define(&mut self, binding: Binding) -> Result<(), String> {
        if self.bindings.iter().any(|b| b.name == binding.name) {
            return Err(format!(
                "duplicate binding `{}` in the same scope",
                binding.name
            ));
        }
        self.bindings.push(binding);
        Ok(())
    }

    #[must_use]
    pub fn lookup_local(&self, name: &str) -> Option<&Binding> {
        self.bindings.iter().rev().find(|b| b.name == name)
    }
}

/// Stack of scopes; top is innermost.
#[derive(Debug, Clone)]
pub struct ScopeStack {
    scopes: Vec<Scope>,
}

impl ScopeStack {
    #[must_use]
    pub fn new_module() -> Self {
        Self {
            scopes: vec![Scope::new()],
        }
    }

    pub fn push(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub fn pop(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn define(&mut self, binding: Binding) -> Result<(), String> {
        self.scopes
            .last_mut()
            .expect("scope stack non-empty")
            .define(binding)
    }

    #[must_use]
    pub fn resolve(&self, name: &str) -> Option<&Binding> {
        for scope in self.scopes.iter().rev() {
            if let Some(b) = scope.lookup_local(name) {
                return Some(b);
            }
        }
        None
    }

    #[must_use]
    pub fn depth(&self) -> usize {
        self.scopes.len()
    }
}
